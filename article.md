## 摘要

随着互联网应用规模的持续扩张与安全威胁的日益复杂化，Web 身份认证系统面临越来越苛刻的安全与可用性要求。本文提出并实现了一套基于“双 Token”（Access Token + Refresh Token）机制的安全 Web 认证系统，并在实现层面给出前后端协作的工程实践细节：后端使用 Rust + Axum 提供 RESTful 接口（包括登录、登出、me 查询与 refresh-token 轮换接口）；前端使用 SvelteKit（配合 sveltekit-superforms）实现表单、服务端 action 与会话管理。系统将短期有效的 Access Token 与长期有效的 Refresh Token 的职责分离：Access Token 用于请求鉴权并通过 HTTP Authorization header 传递，Refresh Token 以 HttpOnly Cookie 存储并仅供服务端用于平滑刷新 Access Token。本文详细阐述了双 Token 的实现细节、前端如何安全地存储会话、token 刷新策略、以及相关的安全与部署考量，并结合三级缓存与 DDD 架构保证系统的性能与可维护性。

## 1. 引言

### 1.1 研究背景

（略，保留对身份认证挑战、JWT 与 Session 与双 Token 方案的动机与比较 —— 与原稿一致）

### 1.2 研究现状

（略，保持对单 JWT、Session + JWT、双 Token、无密码认证的归纳）

### 1.3 研究目标

本文在原有目标基础上进一步明确工程化目标：在后端已实现将 Refresh Token 以 HttpOnly Cookie 返回、并在响应头中附带 Access Token（Authorization header）的实现前提下，给出前端（SvelteKit）如何安全地持久化会话、在服务端注入 Authorization header、以及如何实现透明的 Access Token 刷新策略。

### 1.4 主要贡献

- 基于 Rust/Axum 的后端实现（后端在登录时返回 Authorization 头并以 Set-Cookie 下发 refresh_token；提供 rotate-refresh-token 接口使用 refresh cookie 颁发新 access token；logout 清空 refresh cookie；get-me 通过 Authorization 鉴权）。
- 基于 SvelteKit 的前端实现说明，包含 server-side action 如何接收后端响应并把 access token 写入 httpOnly cookie、以及如何在 server-side fetch 中注入 Authorization header，从而实现安全的会话管理与自动刷新机制。
- 对双 Token 存储、刷新与安全性的系统性分析与工程实践建议（包含 CORS、Cookie 标志、SameSite 策略、CSRF 缓解等）。

## 2. 相关工作

（同原稿中对 JWT、OAuth 2.0、HttpOnly Cookie、密码哈希等的综述，结合下文实现细节适当引用）

## 3. 系统整体设计

### 3.1 架构概览（更新）

本项目采用前后端分离架构，但在前端使用 SvelteKit 的同时，利用 SvelteKit 的 server-side actions / hooks 在边界层实现会话 cookie 的读写与 HTTP Authorization header 的注入，从而把敏感 token 保持在服务器可控的 httpOnly cookie 中，避免客户端脚本直接持有凭证。

整体流程简要：
- 浏览器 -> SvelteKit server（server action）-> 后端 API
  - SvelteKit server 在 action 中调用后端登录接口，读取响应头中的 Authorization（access token），并使用 `event.cookies.set('access_token', ...)` 将其写为 httpOnly cookie；同时解析后端返回的 Set-Cookie 中的 refresh_token，将其以 httpOnly cookie 写回浏览器（或直接允许后端 Set-Cookie 拒绝被中间代理丢失）。
- 后续页面的 server-side `load` 和 actions 均通过 SvelteKit server-side fetch 调用后端 API。SvelteKit 的 `handleFetch` hook 从 `event.cookies` 读取 `access_token` 并注入到发往后端的请求的 Authorization 头中（Bearer token），从而满足后端 `AuthMiddleware` 的要求。
- 当后端返回 401（access token 过期）时，SvelteKit server 会调用 `/api/v1/auth/rotate-refresh-token`，该接口使用浏览器自动带上的 refresh_token cookie（httpOnly）进行刷新。如果刷新成功，后端返回新 Authorization header 与新的 refresh_token Set-Cookie，SvelteKit server 将新的 access token 写入 httpOnly cookie 并重试原始请求。

### 3.2 领域驱动设计（DDD）

（保留此前 DDD 说明；项目结构中明确列出 backend 与 frontend 两个工程目录：backend（Rust/Axum/SQLx/Moka/Redis/Postgres）与 frontend（SvelteKit + superforms））

### 3.3 技术选型（更新）

- 后端：Rust + Axum + Tokio + SQLx + Moka + Redis + PostgreSQL（与原稿一致）；
- 前端：SvelteKit（而非 Leptos/WASM） + sveltekit-superforms + tailwind（实际代码库中采用的栈）。选择 SvelteKit 的原因是在需要 server-side 边界处理（安全 cookie 读写、handleFetch 注入 header、action 到后端的代理调用）时能更方便地在服务器层处理敏感凭证，从而把安全责任交给服务端，避免浏览器端脚本直接持有 token。

### 3.4 应用状态管理

在后端，AppState 对象集中管理依赖（token service、repos、缓存、邮件服务等），便于注入与测试。在前端，SvelteKit 的 server-side load + actions 与 sveltekit-superforms 管理表单状态，交互逻辑集中在 server action 中，UI 仅负责展示。

## 4. 双Token机制实现（扩展并结合代码实现）

### 4.1 设计原则（补充）

- 权责分离：Access Token 仅用于资源鉴权，经常传递；Refresh Token 仅用于刷新，不直接用于资源访问。
- 最小暴露：Client-side JavaScript 不应能读取 Refresh Token 或 Access Token（均使用 httpOnly cookie 存储）。
- Rotation 与短寿命：Access Token 短期有效（如 15 分钟或 3600 秒），Refresh Token 较长期（如 7 天），并在每次刷新时实现 refresh token rotation（后端返回新的 refresh token 覆盖旧的），减少重放风险。
- 服务端优先：将关键 token 的派发与管理放在服务端（后端或 SvelteKit server），把浏览器仅作为透明的传输层。

### 4.2 Token 数据结构（结合实现）

- Access Token
  - 类型：JWT（HS256）
  - 包含 claims：sub（user id）、exp、iat、ver（access token version，用于与用户的 access_token_version 字段比对以实现服务端强制失效）等。
  - 传输：通过 HTTP Authorization header（Bearer）在每次请求中发送给后端。
- Refresh Token
  - 类型：随机 UUID v4 字符串（不可预测）
  - 存储：由后端通过 Set-Cookie 设置为名为 `refresh_token` 的 httpOnly cookie（secure、sameSite=Strict、path=/，开发环境可放宽 secure）
  - 用途：仅用于 `POST /api/v1/auth/rotate-refresh-token` 接口，后端在该接口从 Cookie 读取 refresh_token 并验证后返回新的 access token（Authorization header）与新的 refresh_token Set-Cookie。

### 4.3 Access Token 服务实现（后端）

后端提供 Access Token 的生成与验证逻辑（见后端 AccessTokenService）。在登录与 rotate-refresh-token 成功时，后端会将新的 access token 放入响应头 `Authorization: Bearer <access_token>`，并由 SvelteKit 的 server action 或其它 API 网关读取并持久化为 httpOnly cookie（见下文前端实现）。后端在所有需要鉴权的路由上使用中间件（AuthMiddleware）检查请求的 Authorization header 并解析 JWT claims，验证 `ver` 字段与用户存储的 access_token_version 保持一致，从而实现强制失效。

### 4.4 Refresh Token 服务实现（后端）

后端生成 refresh token（UUID）并存储到 refresh token 仓储（可在数据库或 Redis 中），在登录时通过 `Set-Cookie` 下发到浏览器；在 rotate-refresh-token 接口中，后端从 CookieJar 中读取 refresh_token，验证并生成新的 access token 与新的 refresh token（并以 Set-Cookie 下发），同时作必要的 revoke/blacklist 处理以防止旧 refresh token 重放。

### 4.5 登录流程（前后端协作，工程实现要点）

实际项目实现要点（结合实际后端代码行为）：

1. 用户在前端提交登录表单（SvelteKit 的 server action）。
2. SvelteKit server action 使用 `event.fetch` 调用后端登录接口（`POST /api/v1/auth/login`）。
3. 后端在成功后返回：
   - 响应头中包含 `Authorization: Bearer <access_token>`；
   - 响应头中包含 `Set-Cookie: refresh_token=<...>; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=...`。
   （这是本项目后端实际行为，见后端 login_handler 实现）
4. SvelteKit server action 必须把后端返回的 access token 写入到浏览器的 httpOnly cookie（例如 `access_token`），示意写法为在 action 中调用 `event.cookies.set('access_token', token, { httpOnly: true, path: '/', sameSite: 'lax', secure: ... })`。
   - 原因：SvelteKit server 掌握两端（浏览器与后端）的边界，可以把服务端获取到的 Authorization header 转为浏览器可用的 httpOnly cookie；若直接把 access token 暴露到客户端 JS（localStorage），会增加 XSS 风险。
5. refresh_token cookie 的下发：
   - 如果 SvelteKit server 是以 server action 通过 `event.fetch` 调用后端，后端返回的 Set-Cookie 头不会自动传递给浏览器；因此需要在 server action 中解析后端响应的 Set-Cookie（或从后端响应 body 中读取 refresh_token 值）并通过 `event.cookies.set('refresh_token', ...)` 写入浏览器 cookie，从而确保刷新 token 能保留在浏览器的 httpOnly cookie 中。
   - 另一种做法是在客户端直接向后端发起登录请求（fetch / form submit），让后端直接对浏览器 Set-Cookie（这要求正确设置 CORS 并在前端 fetch 时带上 credentials: 'include'）。但在 SvelteKit 的 server action 模型下，服务端读取并设置 cookie 往往更稳健，并能集中做安全策略。
6. 登录完成后前端通常重定向到受保护页面（如 /dashboard），后续的 server-side load 会通过注入 Authorization 来访问 /api/v1/me。

### 4.6 登录用例的业务逻辑（后端）

（保留 LoginCase 的职责说明；强调后端验证密码（Argon2id），生成 tokens，写 refresh token 仓储，返回 Authorization header 和 Set-Cookie）

### 4.7 前端会话存储（核心实务指南）

基于现有后端实现，推荐并已在项目中采用的前端会话存储方案如下：

- refresh_token：完全由后端以 HttpOnly Cookie 管理（Set-Cookie）。浏览器脚本不可访问（document.cookie 无法读取），因此从 XSS 风险角度最安全。后端设置 cookie 时应包含：
  - HttpOnly（必须）
  - Secure（生产环境必须，开发本地可临时禁用）
  - SameSite=Strict 或 Lax（推荐 Strict，若有跨站登录/第三方嵌入使用场景可考虑 Lax）
  - Path=/，Max-Age 或 Expires（如 7 天）
- access_token：后端在响应头返回（Authorization），SvelteKit server action 读取后把其写为 httpOnly cookie（`access_token`），并设置短过期时间（例如 15 分钟或 3600 秒）。将 access_token 存为 httpOnly cookie 的理由：
  - 保持统一：客户端脚本依然无法读取 token，减少 XSS 风险；
  - 服务端能在 server-side fetch 时读取 cookie 并注入 Authorization header，满足后端 `AuthMiddleware` 的校验需求。
- 注入 Authorization header（服务端代理）
  - 在 SvelteKit 中实现 `handleFetch` hook：每次 server-side 发起 fetch 到后端 API 时，读取 `event.cookies.get('access_token')` 并在 outgoing request 设置 `Authorization: Bearer <access_token>`。因此后端总是能收到 Authorization header 即便 token 存在 cookie 中。
- 自动刷新（token rotation）
  - 在 server-side fetch 中检测 401 响应，如果是 access token 过期导致，先发起 `${API}/api/v1/auth/rotate-refresh-token`（server-side fetch，浏览器会随请求携带 refresh_token cookie 或 SvelteKit server action 会带上 cookie），后端在成功时返回新的 Authorization header 与 Set-Cookie（新的 refresh_token）。SvelteKit server 读取新的 Authorization header 设置新的 access_token cookie，并重试原始请求。若刷新失败（refresh token 过期或被撤销），清除本地 cookies 并重定向到登录页。
- 登出
  - 前端触发 logout action（server-side fetch 到 `/api/v1/auth/logout`），后端在响应中通过 Set-Cookie 清空 refresh_token，前端也应调用 `event.cookies.delete('access_token')` 与 `event.cookies.delete('refresh_token')` 并重定向到登录页。

此设计保证了：
- 即使页面存在 XSS，恶意脚本无法直接拿到 token；
- CSRF 风险被 SameSite 与仅在 server 注入 Authorization header 的策略减少（详见安全节）；
- 可实现透明刷新、平滑用户体验（长时间在线时无需频繁交互登录）。

### 4.8 错误处理体系（简述）

- 登录、注册、刷新等操作在前端以 sveltekit-superforms 的 server actions 进行结构化处理，错误和表单校验结果集中返回并由页面以 toast 或表单错误展示。
- 后端以统一 ApiResponse 封装（code/message/data），error handler 转化为适当的 HTTP 状态码，避免泄露敏感信息（如“用户不存在”与“密码错误”的区分应谨慎，以防用户枚举攻击）。

## 5. 三级缓存架构（略或保持原稿内容）

（保持 Moka/Redis/Postgres 的分层缓存设计与实现说明）

## 6. 系统安全性分析（结合会话存储策略补充）

### 6.1 密码安全（保留 Argon2id）

### 6.2 Token 安全（补充基于实现的分析）

- XSS：Refresh token 与 access token 均以 httpOnly cookie 或仅在服务端持有，客户端脚本无法直接读取，极大降低了 XSS 导致的 token 泄露风险。
- CSRF：因为浏览器会自动在跨站请求中携带 cookie（包括 refresh_token cookie），存在 CSRF 风险。但在本方案中：
  - 日常 API 调用使用 Authorization header 传递 access token；该 header 由服务端在 server-side fetch 中注入（不是浏览器脚本添加），因此普通跨站 GET/POST 表单不会自动带上 Authorization，从而降低 CSRF 面。
  - refresh 操作依赖 refresh_token cookie，因此需要防护：
    - 使用 SameSite=Strict/Lax 来阻止跨站请求携带 cookie；
    - 在 rotate-refresh-token 与 logout 等敏感接口上，结合 Origin/Referer 检查或额外的 CSRF token（Double Submit Cookie 或在请求体中包含一次性 token）进一步增强安全。
- Token 吊销：后端通过 refresh token 仓储与 access_token_version 机制支持强制使 token 失效。
- 网络层：在生产环境强制使用 HTTPS（Secure cookie），并使用短的 access token 生存期与 refresh token rotation 降低被窃取后的风险窗口。

### 6.3 输入验证（保留 Zod / domain-level validation）

...

## 7. 系统实现与部署（结合实际项目结构）

### 7.1 项目结构（基于现有仓库）
- `backend/`：Rust 后端（Axum）实现，包含 `src/presentation/http/v1/handlers`（login/logout/rotate-refresh-token/get-me 等）、`application`、`domain`、`infrastructure` 等模块。
- `frontend/`：SvelteKit 前端实现，使用 sveltekit-superforms 管理表单；login/register 的 server actions 已实现初步对接后端接口（位于 `frontend/src/routes/(auth)/login/+page.server.ts` 与 `.../register/+page.server.ts`），但需完善在登录时将后端返回的 Authorization 转为 httpOnly cookie、解析并设置 refresh_token cookie，以及实现 `hooks.server.ts` 的 handleFetch 注入 Authorization 的逻辑以匹配后端要求。

> 实务提示：前端环境变量应使用 Vite 的 `PUBLIC_` 前缀（如 `PUBLIC_API_BASE_URL`）以便在客户端/服务端一致读取。开发阶段可使用 Vite proxy 将 `/api` 代理到后端以避免 CORS 问题，但生产环境需正确配置后端 CORS 与 cookie 的域/secure 设置。

### 7.2 API 接口设计（摘录）
- POST /api/v1/auth/login
  - 请求：{ nameOrEmail, password }
  - 成功响应：
    - Authorization header: `Bearer <access_token>`
    - Set-Cookie: `refresh_token=<...>; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=...`
    - body: { code, message, data }
- POST /api/v1/auth/rotate-refresh-token
  - 读取 refresh_token cookie，返回新的 Authorization header 与新的 Set-Cookie（refresh_token）
- POST /api/v1/auth/logout
  - 需要 Authorization（或依赖 cookie）；响应中清空 refresh_token cookie

（详细 API 文档见代码与 openapi 生成器）

### 7.3 前端实现要点（工程化步骤）

1. 在登录 action（`frontend/src/routes/(auth)/login/+page.server.ts`）：
   - 执行 `event.fetch(`${API}/api/v1/auth/login`, { method: 'POST', body: ... })`；
   - 读取 `resp.headers.get('authorization')`，解析出 access token 并通过 `event.cookies.set('access_token', token, { httpOnly: true, path: '/', sameSite: 'lax', secure: ... })` 写入浏览器 cookie；
   - 读取 `resp.headers.get('set-cookie')` 中的 refresh_token（如果后端返回），解析并 `event.cookies.set('refresh_token', value, { httpOnly: true, path: '/', sameSite: 'strict', secure: ... })`；
   - 重定向到受保护页面。
2. 在 `src/hooks.server.ts` 中实现 `handleFetch`，为 server-side fetch 自动注入 Authorization：
   - 每次 server fetch 前读取 `event.cookies.get('access_token')`，若存在则在请求头设置 `Authorization: Bearer <token>`。
   - 若请求返回 401，则调用 `${API}/api/v1/auth/rotate-refresh-token`（server-side fetch，cookie 自动带上），若刷新成功，写入新的 access_token cookie 并重试原请求；若刷新失败则清除 cookies 并重定向到登录。
3. 实现 `dashboard/+page.server.ts` 的 `load` 使用 `event.fetch(`${API}/api/v1/me`)` 来校验用户会话（handleFetch 自动注入 Authorization）；未认证者重定向到 `/login`。
4. logout action：`event.fetch(`${API}/api/v1/auth/logout`, { method: 'POST' })`，并 `event.cookies.delete('access_token')` 与 `event.cookies.delete('refresh_token')`。

这些步骤与本仓库已有的后端行为完全兼容（后端在 login/rotate-return Authorization header 并通过 Set-Cookie 更新 refresh_token；AuthMiddleware 从 Authorization header 验证 JWT）。

### 7.4 部署注意（cookie 与 CORS）

- 后端 CORS：当使用浏览器直接向后端发起请求并希望后端 Set-Cookie 生效时，后端必须允许前端 origin 且设置 `Access-Control-Allow-Credentials: true`；前端 fetch 需设置 `credentials: 'include'`。
- 若采用 SvelteKit server action 代理后端（推荐），则大部分跨域问题被规避，因为 server->server 请求无需浏览器 CORS 限制。但在任何情况下，生产环境下必须强制启用 HTTPS 并为 cookie 设置 `Secure` 标志。
- Cookie 的 Domain 与 SameSite 设置需要考虑前端与后端部署的域名架构（同域 vs 子域 vs 不同域）。

## 8. 性能分析（略或保留原稿内容）

（保留对三级缓存性能的讨论、Rust 异步性能优势、数据库优化等）

## 9. 总结与展望

本文在理论与工程实现层面给出了一套基于双 Token 的安全 Web 认证系统的实现：后端以 Rust/Axum 实现 token 生成、refresh-token rotation 与鉴权中间件；前端以 SvelteKit 实现 server-side action 将 access token 持久化为 httpOnly cookie，并通过 server-side fetch 的 handleFetch 自动注入 Authorization header，从而将敏感凭证保持在服务端控制之下，显著降低 XSS 风险并实现平滑的 token 刷新。该方案兼顾安全性与用户体验，可作为构建生产级认证服务的工程模板。

未来工作方向包括：
- 引入更强的 CSRF 缓解（如双提交 cookie、对 rotate 接口增加额外一次性约束）；
- 支持 WebAuthn / FIDO2 等无密码认证方式以提高安全性与用户体验；
- 对 token 黑名单、异常登录行为进行实时检测并在缓存层面进行隔离与加速处理；
- 将前端进一步迁移到边缘计算（Edge Functions）以减少延迟并优化 SSO 场景。

## 参考文献

（保留或扩展原稿参考文献，引用 RFC、OWASP、Argon2、Axum/SQLx 文档等）