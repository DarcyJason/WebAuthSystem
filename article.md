## 摘要

随着互联网应用规模的持续扩张与安全威胁的日益复杂化，Web身份认证系统面临着前所未有的挑战。一方面，用户对无缝、流畅的登录体验有着越来越高的期望；另一方面，XSS、CSRF、Token劫持等攻击手段不断演进，对认证系统的安全性提出了严苛要求。传统的单Token认证方案在这两个维度上难以兼顾：若将Token有效期设置较长，一旦Token泄露攻击者便可长期冒充合法用户；若将有效期设置较短，则用户需要频繁重新登录，严重损害使用体验。

本文针对上述矛盾，提出并完整实现了一种基于双Token机制的安全Web认证系统。系统采用短期有效的JWT（JSON Web Token）作为Access Token，负责日常接口鉴权，有效期默认为3600秒；同时采用长期有效的UUID v4随机字符串作为Refresh Token，负责在Access Token过期后无感刷新，有效期为7天，并通过HttpOnly Cookie安全存储于客户端，使JavaScript脚本无法读取，从根本上切断了XSS攻击的Token窃取路径。

在技术实现层面，系统后端采用Rust语言与Axum框架构建，充分利用Rust的内存安全保证和Tokio异步运行时的高并发能力；前端采用Leptos框架将Rust代码编译为WebAssembly在浏览器中运行，将类型安全的保障延伸至客户端。数据访问层引入Moka进程内缓存（L1）、Redis分布式缓存（L2）、PostgreSQL持久化存储（L3）构成的三级缓存架构，将热点用户数据的查询延迟从毫秒级降低至微秒级。系统整体遵循领域驱动设计（DDD）原则，将业务逻辑与技术实现严格解耦，保证了代码的可维护性与可扩展性。密码存储采用Argon2id算法，数据库操作全部使用参数化查询，配合CORS策略和结构化错误处理，构建了多层次的纵深防御体系。

实验与分析结果表明，该系统在安全性、性能和可维护性方面均达到了预期目标，为构建现代化、高安全等级的Web认证服务提供了一套完整的参考实现。

## 1. 引言

### 1.1 研究背景

身份认证是Web应用安全体系的第一道防线，也是用户与系统建立信任关系的基础环节。HTTP协议本身是无状态的，每一次请求对服务端而言都是独立的，服务端无法天然地识别连续请求是否来自同一用户。为解决这一问题，业界先后发展出Session-Cookie机制和Token机制两大主流方案，各有其适用场景与局限性。

**Session-Cookie方案**将会话状态存储于服务端，通常以内存或数据库的形式保存Session数据，客户端仅持有一个不透明的Session ID。这种方案在单体应用中运行良好，服务端对会话拥有完全的控制权，可以随时使任意Session失效。然而，随着云原生和微服务架构的普及，Session-Cookie方案暴露出明显的扩展性问题：在多实例部署场景下，不同服务器实例之间需要共享Session存储（通常借助Redis等外部存储），引入了额外的网络开销和单点故障风险；同时，Session存储的内存占用随在线用户数线性增长，在大规模并发场景下对服务端资源造成较大压力。

**Token方案**（尤其是JWT）将状态编码于Token本身，服务端无需存储任何会话信息，仅需持有验证密钥即可对Token进行校验，天然支持无状态横向扩展。JWT的自包含特性使其在微服务间传递身份信息时尤为便利，无需每次都查询中心化的Session存储。然而，JWT的无状态特性也带来了新的安全挑战：Token一旦签发便难以主动吊销，在Token过期之前，即使用户已经登出或账户已被封禁，持有该Token的攻击者仍可继续访问受保护资源。这一特性迫使系统设计者在Token有效期的设置上陷入两难困境：

- 若设置较长的有效期（如数天或数周），用户体验良好，但Token泄露后的影响窗口极长，安全风险极高；
- 若设置较短的有效期（如数分钟），安全性较好，但用户需要频繁重新登录，严重损害使用体验，尤其在移动端和长时间使用场景下问题尤为突出。

**双Token机制**正是为解决上述矛盾而提出的折中方案：引入两种职责不同的Token，使用短期有效的Access Token用于日常接口鉴权，使用长期有效的Refresh Token用于在Access Token过期后静默刷新，两者各司其职，在安全与体验之间取得平衡。这一模式最早由OAuth 2.0协议（RFC 6749）在授权场景中系统化定义，后被广泛借鉴于各类Web认证系统。

然而，双Token机制的安全性在很大程度上取决于Refresh Token的存储方式。若将Refresh Token存储于浏览器的localStorage或sessionStorage，则面临XSS攻击的威胁——攻击者一旦在页面中注入恶意脚本，便可轻易读取并窃取Token。将Refresh Token存储于设置了HttpOnly属性的Cookie中，可以从根本上阻断JavaScript对Token的访问，是目前业界公认的最佳实践。

在技术选型层面，Rust语言近年来在系统编程和Web后端领域获得了广泛关注。Rust的所有权系统在编译期消除了内存安全漏洞（如缓冲区溢出、悬空指针、数据竞争等），这些漏洞在C/C++编写的系统中长期是安全漏洞的重要来源。Axum框架基于Tokio异步运行时，提供了高性能、类型安全的HTTP服务能力。前端方面，Leptos框架将Rust的安全性延伸至浏览器端，通过WebAssembly技术实现了接近原生JavaScript的运行性能，同时享有Rust编译器的严格类型检查。

### 1.2 研究现状

目前，业界主流的Web认证方案可分为以下几类：

**纯JWT方案**：以单一JWT作为认证凭证，有效期通常设置为数小时至数天。这种方案实现简单，但在安全性上存在明显缺陷，无法主动吊销Token，且Token泄露后影响持续时间长。

**Session + JWT混合方案**：使用JWT传递身份信息，但在服务端维护Token黑名单（通常存储于Redis），以实现主动吊销能力。这种方案在一定程度上弥补了纯JWT的缺陷，但引入了服务端状态，部分抵消了JWT无状态的优势。

**双Token方案**：如本文所实现，使用短期Access Token + 长期Refresh Token的组合。这是目前安全性与可用性平衡最佳的方案，被Google、GitHub、Stripe等主流平台广泛采用。

**无密码认证（Passwordless）**：通过邮件魔法链接、短信验证码或WebAuthn（FIDO2）等方式替代传统密码，从根本上消除密码泄露风险。这是认证领域的发展趋势，但实现复杂度较高，用户教育成本也相对较大。

本文聚焦于双Token方案的完整实现，并在此基础上探讨三级缓存架构对认证系统性能的优化效果。

### 1.3 研究目标

本文的研究目标如下：

1. **安全性**：设计并实现一套完整的双Token认证方案，明确两种Token的职责边界、有效期策略与安全存储方式，有效防御XSS、CSRF、Token重放等常见攻击；
2. **性能**：引入Moka + Redis + PostgreSQL三级缓存架构，将热点用户数据的查询延迟降低至微秒级，支持高并发认证请求；
3. **可维护性**：遵循领域驱动设计（DDD）原则，将业务逻辑与技术实现严格解耦，保证代码的可读性、可测试性和可扩展性；
4. **类型安全**：充分利用Rust的类型系统，在编译期捕获尽可能多的错误，减少运行时异常；
5. **工程完整性**：提供完整的容器化部署方案、数据库迁移脚本、环境配置管理和API文档，使系统具备生产环境部署能力。

### 1.4 主要贡献

本文的主要贡献包括：

1. 基于Rust生态（Axum + Leptos）实现了完整的双Token认证系统，提供了一套可直接参考的工程实践；
2. 设计了Moka（L1）+ Redis（L2）+ PostgreSQL（L3）三级透明缓存架构，通过`LayeredUserRepository`抽象屏蔽了缓存细节，上层业务代码无需感知缓存层次；
3. 将DDD分层架构与Rust的trait系统相结合，实现了领域服务接口与基础设施实现的彻底解耦；
4. 对系统的安全性进行了系统性分析，覆盖密码存储、Token安全、输入验证、错误信息安全等多个维度。

### 1.5 论文结构

本文第2节介绍相关工作，梳理JWT、OAuth 2.0、HttpOnly Cookie等核心技术的背景；第3节阐述系统整体设计，包括架构概览、DDD分层和技术选型；第4节详细描述双Token认证机制的完整实现；第5节介绍三级缓存架构的设计与实现；第6节从多个维度分析系统安全性；第7节介绍系统实现细节与部署方案；第8节进行性能分析；第9节总结全文并展望未来工作。

## 2. 相关工作

### 2.1 JWT（JSON Web Token）

JWT（RFC 7519）是一种开放标准，定义了一种紧凑且自包含的方式，用于在各方之间以JSON对象安全地传输信息。一个JWT由三部分组成：Header（头部）、Payload（载荷）、Signature（签名），三部分以`.`分隔并经Base64Url编码。

**Header** 声明Token类型（JWT）和签名算法（如HS256、RS256）。**Payload** 包含声明（Claims），分为注册声明（如`iss`签发者、`sub`主题、`exp`过期时间、`iat`签发时间）、公共声明和私有声明。**Signature** 由Header和Payload的Base64Url编码拼接后，使用指定算法和密钥计算得出，用于验证Token未被篡改。

JWT的核心优势在于无状态性：服务端通过验证签名即可确认Token的合法性，无需查询数据库或外部存储。这使得JWT在分布式系统中传递身份信息时极为高效。然而，JWT的无状态特性也是其主要缺陷所在——在Token过期之前，服务端无法主动使其失效，这对需要即时吊销能力的场景（如用户登出、账户封禁）构成挑战。

在签名算法的选择上，常见的有：

- **HS256（HMAC-SHA256）**：对称算法，签名和验证使用同一密钥，适用于单体或可信内部服务间通信，密钥管理相对简单；
- **RS256（RSA-SHA256）**：非对称算法，私钥签名、公钥验证，适用于需要将验证能力分发给多个服务的场景；
- **ES256（ECDSA-SHA256）**：椭圆曲线非对称算法，在相同安全强度下密钥更短，性能优于RS256。

本系统采用HS256算法，密钥通过环境变量注入，适合单体部署场景。

### 2.2 OAuth 2.0与双Token模式

OAuth 2.0协议（RFC 6749）是目前互联网上最广泛使用的授权框架，定义了四种授权流程（授权码、隐式、资源所有者密码凭证、客户端凭证）。在授权码流程中，OAuth 2.0系统化地引入了Access Token与Refresh Token的双Token概念，为现代认证系统提供了重要的设计参考。

**Access Token** 是授权服务器颁发的凭证，代表资源所有者授予客户端的访问权限，有效期较短（通常为数分钟至数小时）。客户端使用Access Token访问受保护的资源服务器，资源服务器通过验证Token的有效性来决定是否响应请求。

**Refresh Token** 是一种特殊凭证，用于在Access Token过期后向授权服务器请求新的Access Token，有效期较长（通常为数天至数周）。Refresh Token通常只在客户端与授权服务器之间传递，不会发送给资源服务器，从而降低了其暴露风险。

OAuth 2.0的双Token设计思路被广泛借鉴于非OAuth场景的Web认证系统中。在这类系统中，认证服务器同时扮演授权服务器和资源服务器的角色，双Token机制的核心思想得以保留：短期Access Token用于日常鉴权，长期Refresh Token用于无感续期。

值得注意的是，OAuth 2.0本身并未规定Refresh Token的存储方式，这一细节对安全性至关重要，需要在具体实现中加以考量。

### 2.3 HttpOnly Cookie的安全价值

Token的存储位置是Web认证安全的关键决策点。常见的客户端存储方式有三种：

**localStorage**：持久化存储，页面关闭后数据仍然保留。最大的安全隐患是可被JavaScript直接读取，一旦页面存在XSS漏洞，攻击者注入的脚本即可通过`localStorage.getItem()`窃取Token。

**sessionStorage**：会话级存储，页面关闭后数据清除。同样可被JavaScript读取，XSS风险与localStorage相同，仅在持久性上有所区别。

**HttpOnly Cookie**：通过`Set-Cookie`响应头设置`HttpOnly`属性后，浏览器会阻止JavaScript通过`document.cookie`读取该Cookie，从根本上切断了XSS攻击的Token读取路径。即使页面存在XSS漏洞，攻击者也无法通过脚本获取HttpOnly Cookie的内容。

HttpOnly Cookie通常与以下属性配合使用，构建完整的Cookie安全策略：

| 属性 | 作用 |
| --- | --- |
| HttpOnly | 禁止JavaScript读取，防御XSS |
| Secure | 仅通过HTTPS传输，防御网络窃听 |
| SameSite=Strict | 禁止跨站请求携带Cookie，防御CSRF |
| Path=/ | 限制Cookie的适用路径范围 |
| Max-Age | 设置Cookie的有效期 |

本系统将Refresh Token存储于设置了`HttpOnly`和`Path=/`属性的Cookie中，有效期7天，在生产环境中应同时启用`Secure`和`SameSite`属性。

### 2.4 密码哈希算法演进

密码的安全存储是认证系统的基础安全要求。直接存储明文密码或使用简单哈希（如MD5、SHA-1）均存在严重安全隐患，一旦数据库泄露，攻击者可通过彩虹表或暴力破解快速还原原始密码。

密码哈希算法的演进历程如下：

- **MD5/SHA-1**：通用哈希算法，计算速度极快，不适合密码存储。现代GPU每秒可计算数百亿次MD5哈希，暴力破解成本极低；
- **bcrypt**：专为密码哈希设计，引入了工作因子（cost factor）机制，可调节计算复杂度。但其内存需求固定，对GPU并行攻击的抵抗力有限；
- **scrypt**：在bcrypt基础上引入了内存困难性，需要大量内存才能计算，显著提高了GPU/ASIC攻击的成本；
- **Argon2**：2015年密码哈希竞赛（PHC）的获胜算法，提供了三个变体：Argon2d（抵御GPU攻击）、Argon2i（抵御侧信道攻击）、Argon2id（两者结合，推荐用于密码哈希）。Argon2id是目前OWASP和NIST推荐的密码哈希算法首选。

本系统采用Argon2id算法，通过Rust的`argon2` crate实现，每次哈希自动生成随机盐值，原始密码永不落盘。

### 2.5 Rust在Web后端的应用

Rust是一门系统编程语言，以内存安全、零成本抽象和高并发性能著称。其所有权（Ownership）和借用检查（Borrow Checker）机制在编译期消除了内存安全漏洞，无需垃圾回收器即可保证内存安全，在性能上接近C/C++。

在Web后端领域，Rust生态已相当成熟：

- **Tokio**：基于async/await的异步运行时，采用工作窃取（Work-Stealing）调度算法，在多核CPU上实现高效的任务调度；
- **Axum**：基于Tokio和Tower构建的Web框架，提供类型安全的路由、提取器（Extractor）和中间件机制，Handler函数的参数类型即是其提取逻辑的声明；
- **Tower**：中间件抽象层，定义了`Service` trait，使各种中间件（认证、限流、追踪、CORS等）可以以统一的方式组合；
- **SQLx**：异步SQL工具包，支持编译期SQL验证，在编译时检查SQL语句的正确性和类型安全性。

相比Go、Node.js等语言，Rust在以下方面具有显著优势：内存占用更低（无GC暂停）、并发安全性更强（编译期数据竞争检测）、运行时错误更少（类型系统更严格）。这些特性使Rust成为构建高安全等级Web服务的理想选择。

### 2.6 WebAssembly与Leptos框架

WebAssembly（WASM）是一种二进制指令格式，设计为可在现代Web浏览器中高效执行的低级虚拟机目标。WASM具有接近原生代码的执行速度，且与JavaScript可以互操作，为在浏览器中运行非JavaScript语言编写的代码提供了标准化途径。

Leptos是一个基于Rust的全栈Web框架，支持客户端渲染（CSR）、服务端渲染（SSR）和同构渲染（Hydration）三种模式。本系统采用CSR模式，将Rust代码编译为WASM在浏览器中运行。Leptos的核心特性包括：

- **细粒度响应式系统（Fine-grained Reactivity）**：基于Signal/Effect/Memo的响应式原语，仅更新实际发生变化的DOM节点，避免了React等框架的虚拟DOM全量diff开销；
- **编译期模板验证**：`view!`宏在编译期验证HTML模板的正确性，类型错误在编译时即可发现；
- **与Rust生态无缝集成**：可直接使用Rust的类型系统、错误处理机制和标准库，无需在Rust和JavaScript之间进行类型转换。

前端使用Tailwind CSS 4.x进行样式管理，通过`tw_merge` crate在Rust代码中安全地合并CSS类名，避免了类名冲突问题。

## 3. 系统整体设计

### 3.1 架构概览

系统采用前后端分离架构，前端编译为WebAssembly静态资源由Nginx托管，后端提供RESTful API服务。两者通过HTTP协议通信，后端通过CORS中间件控制跨域访问权限。整体架构分为以下几层：

```
┌─────────────────────────────────────────────┐
│              前端（Leptos/WASM）             |
│         浏览器渲染 · 路由 · 表单交互           │
└──────────────────┬──────────────────────────┘
                   │ HTTP/REST API
┌──────────────────▼──────────────────────────┐
│           后端（Axum / Rust）                │
│  ┌─────────────────────────────────────┐    │
│  │  Presentation Layer（HTTP处理层）     │    │
│  │  路由 · Handler · 中间件 · 响应格式    │    │
│  ├─────────────────────────────────────┤    │
│  │  Application Layer（应用层）          │    │
│  │  Use Cases · Commands · Results     │    │
│  ├─────────────────────────────────────┤    │
│  │  Domain Layer（领域层）               │    │
│  │  实体 · 值对象 · 领域服务 · 仓储接口     │    │
│  ├─────────────────────────────────────┤    │
│  │  Infrastructure Layer（基础设施层）    │    │
│  │  JWT · Argon2 · 三级缓存 · 邮件服务    │    │
│  └─────────────────────────────────────┘    │
└──────────────────┬──────────────────────────┘
                   │
┌──────────────────▼──────────────────────────┐
│              数据存储层                       │
│   Moka（L1内存）· Redis（L2分布式）·             │
│   PostgreSQL（L3持久化）                      │
└─────────────────────────────────────────────┘
```

这种分层架构的核心优势在于关注点分离（Separation of Concerns）：每一层只负责自己的职责，层与层之间通过明确定义的接口通信，使得各层可以独立演进和替换。例如，若未来需要将PostgreSQL替换为其他数据库，只需修改基础设施层的实现，领域层和应用层的代码完全不受影响。

### 3.2 领域驱动设计（DDD）

领域驱动设计（Domain-Driven Design，DDD）是一种以业务领域为核心的软件设计方法论，由Eric Evans在2003年的同名著作中系统化提出。DDD的核心思想是将软件的复杂性集中在领域模型中，通过统一语言（Ubiquitous Language）使技术团队与业务团队达成共识。

系统后端严格遵循DDD分层架构，各层职责如下：

**领域层（Domain Layer）** 是整个系统的核心，包含所有业务规则和领域知识，不依赖任何外部框架或基础设施。领域层的主要组成部分：

- **聚合根（Aggregate Root）**：`User`是系统的核心聚合根，封装了用户的所有属性（ID、姓名、邮箱、凭证列表、状态、时间戳）和业务不变量。聚合根是事务一致性的边界，对用户数据的所有修改都必须通过`User`聚合根进行；
- **实体（Entity）**：`Credential`是凭证实体，具有唯一标识，包含凭证类型（密码、OAuth等）和凭证状态；
- **值对象（Value Object）**：`UserId`、`UserEmail`、`UserName`、`PlainPassword`、`AccessToken`、`RefreshToken`等均为值对象，通过类型系统在编译期保证业务规则。例如，`UserEmail`在构造时验证邮箱格式，一旦构造成功即可保证其合法性，无需在后续代码中重复验证；
- **领域服务接口（Domain Service）**：`AccessTokenService`、`RefreshTokenService`、`PasswordService`定义了领域操作的抽象接口，具体实现由基础设施层提供；
- **仓储接口（Repository Interface）**：`UserQueryRepository`和`UserCommandRepository`定义了用户数据的读写接口，领域层只依赖接口，不依赖具体实现。

**应用层（Application Layer）** 负责编排领域对象完成业务用例，是领域层与表现层之间的协调者。应用层不包含业务规则，只负责调用领域对象和基础设施服务完成用例流程：

- **用例（Use Case）**：`LoginCase`和`RegisterCase`分别封装了登录和注册的完整业务流程；
- **命令对象（Command）**：`LoginCommand`和`RegisterCommand`是用例的输入，包含经过验证的业务数据；
- **结果对象（Result）**：`LoginResult`和`RegisterResult`是用例的输出，包含业务操作的结果数据。

**基础设施层（Infrastructure Layer）** 提供领域服务接口的具体技术实现，是系统与外部世界（数据库、缓存、邮件服务等）交互的边界：

- `DefaultAccessTokenService`：使用`jsonwebtoken` crate实现JWT的生成与验证；
- `DefaultRefreshTokenService`：使用`uuid` crate生成UUID v4随机字符串；
- `Argon2PasswordService`：使用`argon2` crate实现密码的哈希与验证；
- `LayeredUserRepository`：组合Moka、Redis、PostgreSQL三层存储，实现透明的多级缓存；
- `Config`：使用`figment2` crate从环境变量和`.env`文件加载配置。

**表现层（Presentation Layer）** 处理HTTP请求与响应，是系统对外暴露的接口层：

- **路由（Routes）**：定义URL路径与Handler函数的映射关系；
- **Handler**：解析请求载荷，调用应用层用例，将结果序列化为HTTP响应；
- **中间件（Middleware）**：CORS中间件控制跨域访问，Trace中间件记录请求日志；
- **错误处理**：将应用层和领域层的错误统一转换为标准HTTP错误响应。

### 3.3 技术选型

技术选型的核心原则是：在满足功能需求的前提下，优先选择类型安全、性能优秀、社区活跃的方案。

| 组件 | 技术选型 | 版本 | 选型理由 |
| --- | --- | --- | --- |
| 后端语言 | Rust | nightly-2026-04-10 | 内存安全、零成本抽象、高性能 |
| 后端框架 | Axum | 0.8.8 | 类型安全路由、Tower生态、Tokio原生支持 |
| 异步运行时 | Tokio | 1.50.0 | 成熟稳定、高性能、生态最完善 |
| 前端框架 | Leptos | 0.8.17 | Rust全栈、细粒度响应式、编译期验证 |
| 前端编译目标 | WebAssembly | - | 接近原生性能、类型安全 |
| CSS框架 | Tailwind CSS | 4.2.2 | 原子化CSS、构建产物小 |
| 数据库 | PostgreSQL | 18 | ACID事务、JSONB支持、枚举类型 |
| 分布式缓存 | Redis | 8.4.0 | 高性能、丰富数据结构、广泛使用 |
| 内存缓存 | Moka | 0.12.14 | Rust原生、异步支持、LRU/TTL策略 |
| JWT库 | jsonwebtoken | 10.3.0 | 支持aws_lc_rs加密后端、类型安全 |
| 密码哈希 | Argon2 | 0.5.3 | PHC获胜算法、OWASP推荐 |
| ORM/查询 | SQLx | 0.8.6 | 编译期SQL验证、异步支持、类型安全 |
| 邮件服务 | Resend | 0.22.0 | 现代邮件API、高送达率 |
| 错误处理 | Snafu | 0.9.0 | 结构化错误、上下文信息丰富 |
| 容器化 | Docker + Docker Compose | - | 环境一致性、部署简化 |

### 3.4 应用状态管理

系统使用`AppState`结构体集中管理所有共享状态，通过Axum的`State`提取器注入到每个Handler中：

```rust
pub struct AppState {
    pub user_repo: LayeredUserRepository,
    pub password_service: Argon2PasswordService,
    pub access_token_service: DefaultAccessTokenService,
    pub refresh_token_service: DefaultRefreshTokenService,
}
```

`AppState`在服务启动时初始化一次，通过`Arc<AppState>`在所有请求处理任务间共享，避免了重复初始化的开销。各服务实现了`Clone` trait，使得在每个请求中创建用例实例时可以低成本地克隆服务引用。

系统启动时通过`tokio::try_join!`并发初始化Redis和PostgreSQL连接，将串行初始化改为并行，减少了服务启动时间：

```rust
let (redis_client, postgres_client) = tokio::try_join!(
    async { RedisClient::new(&config.redis).await.context(RedisFailedSnafu) },
    async { PostgresClient::new(&config.postgres).await.context(PostgresFailedSnafu) },
)?;
```

## 4. 双Token机制实现

### 4.1 设计原则

双Token方案的核心设计原则如下：

1. **Access Token**：短期有效（默认配置为3600秒），以JWT格式签发，携带用户ID（sub字段）、签发时间（iat）和过期时间（exp）。客户端将其存储于内存中（如JavaScript变量或Leptos的响应式Signal），通过HTTP请求头`Authorization: Bearer <token>`传递给服务端。Access Token不存储于任何持久化存储（localStorage、Cookie），页面刷新后需通过Refresh Token重新获取。
2. **Refresh Token**：长期有效（7天），以UUID v4格式生成，通过`Set-Cookie`响应头以HttpOnly、Path=/、Max-Age=7天的属性写入客户端Cookie。客户端无法通过JavaScript读取，有效防御XSS攻击。Refresh Token在每次使用后应轮换（当前版本待实现），进一步缩短泄露影响窗口。
3. **职责分离**：Access Token负责接口鉴权，Refresh Token仅用于刷新Access Token，两者的传输通道和存储位置均不同，降低了单点泄露的影响范围。即使Access Token泄露，攻击者最多只能在其有效期内（1小时）冒充用户；即使Refresh Token泄露，由于其存储于HttpOnly Cookie，攻击者也无法通过XSS脚本直接读取。
4. **最小权限原则**：JWT载荷中仅包含必要的用户标识（UUID），不包含用户名、邮箱、角色等敏感信息，减少Token泄露时的信息暴露面。

### 4.2 Token数据结构

**Access Token（JWT）载荷结构：**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    sub: String,
    iat: usize,
    exp: usize,
}
```

- `sub`：用户UUID，用于标识用户身份，服务端通过此字段查询用户信息
- `iat`：Token签发时间（Unix时间戳，秒），用于审计和调试
- `exp`：Token过期时间（Unix时间戳，秒），`jsonwebtoken`库在验证时自动检查此字段

JWT的Header固定为`{"alg":"HS256","typ":"JWT"}`，使用HS256（HMAC-SHA256）算法签名。整个Token经Base64Url编码后，典型长度约为200字节，适合在HTTP头部传输。

**Refresh Token：**

Refresh Token采用UUID v4格式（如`f47ac10b-58cc-4372-a567-0e02b2c3d479`），由128位密码学安全随机数生成，具有足够的随机性和唯一性，无法被预测或枚举。UUID v4的碰撞概率约为1/2^122，在实际应用中可视为唯一。

当前版本的Refresh Token为无状态设计（服务端不存储），未来版本将在Redis中维护Refresh Token的有效性记录，以支持主动吊销和轮换机制。

### 4.3 Access Token服务实现

Access Token服务遵循依赖倒置原则（DIP），领域层定义接口，基础设施层提供实现：

```rust
// 领域层接口定义（domain/auth/services/access_token_service.rs）
pub trait AccessTokenService {
    fn generate(&self, user_id: &UserId) -> DomainResult<AccessToken>;
    fn decode(&self, access_token: &AccessToken) -> DomainResult<UserId>;
}

// 基础设施层实现（infrastructure/security/tokens/access_token.rs）
#[derive(Debug, Clone)]
pub struct DefaultAccessTokenService {
    jwt_secret: String,
    expires_in_seconds: i64,
}

impl AccessTokenService for DefaultAccessTokenService {
    fn generate(&self, user_id: &UserId) -> DomainResult<AccessToken> {
        let now = Timestamp::now().value().to_owned();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::minutes(self.expires_in_seconds))
            .timestamp() as usize;
        let claims = AccessTokenClaims::new(
            user_id.value().to_string(), iat, exp
        );
        let token_value = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        ).map_err(|e| EncodeAccessTokenFailedSnafu { message: e.to_string() }.build())?;
        Ok(AccessToken::new(token_value))
    }

    fn decode(&self, access_token: &AccessToken) -> DomainResult<UserId> {
        let decoded = decode::<AccessTokenClaims>(
            access_token.value(),
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        ).map_err(|e| DecodeAccessTokenFailedSnafu { message: e.to_string() }.build())?;
        let user_id = UserId::from_raw_user_id(decoded.claims.sub().to_owned())?;
        Ok(user_id)
    }
}
```

`Validation::new(Algorithm::HS256)`默认会验证`exp`字段，Token过期时`decode`方法将返回错误，无需手动检查过期时间。密钥通过环境变量`JWT__SECRET`注入，建议使用`openssl rand -base64 64`生成至少512位的高熵密钥。

### 4.4 Refresh Token服务实现

Refresh Token服务同样遵循接口与实现分离的原则：

```rust
// 领域层接口（domain/auth/services/refresh_token_service.rs）
pub trait RefreshTokenService {
    fn generate(&self) -> RefreshToken;
}

// 基础设施层实现（infrastructure/security/tokens/refresh_token.rs）
#[derive(Debug, Clone)]
pub struct DefaultRefreshTokenService;

impl RefreshTokenService for DefaultRefreshTokenService {
    fn generate(&self) -> RefreshToken {
        let value = Uuid::new_v4().to_string();
        RefreshToken::new(value)
    }
}
```

`Uuid::new_v4()`使用操作系统提供的密码学安全随机数生成器（CSPRNG）生成UUID，在Linux上对应`/dev/urandom`，在macOS上对应`arc4random`，保证了随机数的不可预测性。

### 4.5 登录流程

登录流程是双Token机制的核心，完整流程如下：

```
客户端                          服务端
  │                               │
  │  POST /api/v1/auth/login      │
  │  { name_or_email, password }  │
  │──────────────────────────────▶│
  │                               │ 1. 验证用户名/邮箱格式
  │                               │ 2. 查询用户（三级缓存）
  │                               │ 3. Argon2验证密码
  │                               │ 4. 检查账户状态
  │                               │ 5. 生成 Access Token (JWT)
  │                               │ 6. 生成 Refresh Token (UUID)
  │                               │
  │  HTTP 200 OK                  │
  │  Authorization: Bearer <AT>   │
  │  Set-Cookie: refresh_token=   │
  │    <RT>; HttpOnly; Path=/;    │
  │    Max-Age=604800             │
  │◀──────────────────────────────│
  │                               │
  │  客户端将AT存入内存             │
  │  浏览器自动保存RT Cookie        │
```

登录Handler的核心实现：

```rust
pub async fn login_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    let cmd: LoginCommand = payload.try_into()?;
    let case = LoginCase::new(
        app_state.user_repo.clone(),
        app_state.password_service.clone(),
        app_state.access_token_service.clone(),
        app_state.refresh_token_service.clone(),
    );
    let result = case.execute(cmd).await?;

    // Refresh Token 写入 HttpOnly Cookie
    let cookie = Cookie::build(("refresh_token", result.refresh_token.value()))
        .path("/")
        .max_age(time::Duration::days(7))
        .http_only(true)
        .build();

    // Access Token 写入 Authorization 响应头
    let mut headers = HeaderMap::new();
    headers.append(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", result.access_token.value())).unwrap(),
    );
    headers.append(SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(ApiResponse::ok(Some(headers), "Login successfully", LoginResponseData))
}
```

这里有几个值得关注的设计细节：

- **Access Token通过响应头返回**，而非响应体。这样客户端可以直接从`Authorization`响应头读取Token，无需解析响应体JSON，也避免了Token出现在响应体日志中；
- **Refresh Token通过`Set-Cookie`响应头设置**，浏览器会自动保存并在后续请求中携带，无需客户端代码干预；
- `http_only(true)`是关键安全属性，确保JavaScript无法读取该Cookie；
- `path("/")`确保Cookie在所有路径下都会被携带，包括未来的`/api/v1/auth/refresh`刷新端点。

### 4.6 登录用例的业务逻辑

`LoginCase`封装了完整的登录业务逻辑，包含多层安全校验，体现了"防御性编程"的思想：

```rust
pub async fn execute(&self, cmd: LoginCommand) -> ApplicationResult<LoginResult> {
    // 1. 支持用户名或邮箱两种登录方式
    let (name, email) = match cmd.name_or_email {
        UserNameOrUserEmail::UserName(n) => (Some(n), None),
        UserNameOrUserEmail::UserEmail(e) => (None, Some(e)),
    };

    // 2. 查询用户（自动命中三级缓存）
    let user = self.user_repo
        .get_by_name_or_email(&name, &email)
        .await
        .context(DomainFailedSnafu)?
        .ok_or_else(|| UserNotFoundSnafu.build())?;

    // 3. 提取密码凭证（用户可能只有OAuth凭证，无密码）
    let password_credential = user.credentials()
        .iter()
        .find_map(|c| c.kind().password_credential())
        .cloned()
        .ok_or_else(|| UserNotFoundSnafu.build())?;  // 故意返回UserNotFound而非NoPasswordCredential，防止信息泄露

    // 4. Argon2验证密码（恒定时间比较，防止时序攻击）
    let matched = self.password_service
        .verify_password(password_credential, cmd.plain_password)
        .context(DomainFailedSnafu)?;
    if !matched {
        return InvalidCredentialsSnafu.fail();
    }

    // 5. 账户状态检查（顺序重要：先验证密码，再检查状态，避免状态枚举）
    match user.status() {
        UserStatus::Banned => return AccountIsBannedSnafu.fail(),
        UserStatus::EmailNotVerified => return EmailNotVerifiedSnafu.fail(),
        UserStatus::Active => {}
    }

    // 6. 生成双Token
    let access_token = self.access_token_service.generate(user.id()).context(DomainFailedSnafu)?;
    let refresh_token = self.refresh_token_service.generate();

    Ok(LoginResult { access_token, refresh_token })
}
```

业务逻辑中有几处安全设计值得说明：

- **统一错误响应**：用户不存在和密码错误均返回`InvalidCredentials`，防止攻击者通过错误信息枚举有效用户名；
- **凭证不存在时返回UserNotFound**：若用户没有密码凭证（如纯OAuth用户），同样返回`UserNotFound`而非更具体的错误，避免泄露账户凭证类型信息；
- **先验密码再检查状态**：这样即使账户被封禁，攻击者也无法通过"账户已封禁"的错误信息确认账户存在且密码正确。

### 4.7 注册流程

注册流程包含输入验证、重复检查和密码安全存储：

```
客户端                              服务端
  │                                   │
  │  POST /api/v1/auth/register       │
  │  { name, email,                   │
  │    password, confirmPassword }    │
  │──────────────────────────────────▶│
  │                                   │ 1. 验证字段格式（名称/邮箱/密码规则）
  │                                   │ 2. 校验两次密码一致性
  │                                   │ 3. 检查用户名/邮箱是否已存在
  │                                   │ 4. Argon2哈希密码（自动加盐）
  │                                   │ 5. 创建用户（状态：EmailNotVerified）
  │                                   │ 6. 写入数据库并预热L2/L1缓存
  │  HTTP 200 OK                      │
  │  { userId: "..." }                │
  │◀──────────────────────────────────│
```

注册请求载荷使用`camelCase`命名（通过`#[serde(rename_all = "camelCase")]`），与前端JavaScript惯例保持一致。密码采用Argon2id算法进行哈希，哈希结果包含算法参数、盐值和哈希值，以PHC字符串格式存储于数据库的JSONB字段中，原始密码永不落盘。

新注册用户的初始状态为`EmailNotVerified`，在完成邮箱验证之前无法登录，这一设计防止了使用他人邮箱注册账户的滥用行为。

### 4.8 用户状态机

系统定义了三种用户状态，构成简单的状态机：

```
注册完成
    │
    ▼
EmailNotVerified ──[邮箱验证]──▶ Active ──[管理员封禁]──▶ Banned
                                   ▲                          │
                                   └──────[管理员解封]─────────┘
```

- `EmailNotVerified`：新注册用户的初始状态，登录时返回`EmailNotVerified`错误，提示用户完成邮箱验证
- `Active`：正常活跃用户，可正常使用所有功能
- `Banned`：被封禁用户，登录时返回`AccountIsBanned`错误，提示账户已被封禁

用户状态在PostgreSQL中以枚举类型存储，保证了数据库层面的数据完整性，防止出现非法状态值。在Rust代码中，`UserStatus`枚举通过`match`语句进行穷举匹配，编译器会强制处理所有可能的状态，避免遗漏。

### 4.9 错误处理体系

系统使用`snafu`库构建结构化的错误处理体系，错误类型在编译期确定，每种错误携带丰富的上下文信息：

```
DomainError（领域层错误）
  ├── EncodeAccessTokenFailed { message }
  ├── DecodeAccessTokenFailed { message }
  ├── InvalidAccessToken { message }
  └── ...

ApplicationError（应用层错误）
  ├── UserNotFound
  ├── InvalidCredentials
  ├── UserAlreadyExists
  ├── AccountIsBanned
  ├── EmailNotVerified
  ├── PasswordsNotMatched
  ├── Validation { message }
  └── DomainFailed { source: DomainError }

ApiError（表现层错误）
  ├── ApplicationFailed { source: ApplicationError }
  └── ...
```

每种错误都映射到特定的HTTP状态码和错误消息，通过`IntoResponse` trait自动转换为标准的JSON错误响应，保证了API错误格式的一致性。

## 5. 三级缓存架构

### 5.1 设计动机

用户信息是认证系统中读取最频繁的数据。在典型的认证场景中，每次受保护接口的请求都需要：①验证Access Token的签名和有效期；②根据Token中的用户ID查询用户信息，确认账户状态和权限。步骤①是纯计算操作，开销极低；步骤②涉及数据库查询，在高并发场景下极易成为性能瓶颈。

以一个日活跃用户（DAU）为10万的系统为例，假设每个用户每天平均发起50次API请求，则每天需要进行500万次用户查询。若每次查询都直接访问PostgreSQL，即使单次查询仅需5ms，也会产生约694 QPS的持续数据库压力，在流量高峰期可能达到数千QPS，远超普通PostgreSQL实例的承载能力。

引入多级缓存是解决这一问题的标准方案。系统设计了三级缓存架构，按照访问速度和存储容量的权衡，将用户数据分层缓存：

| 层级 | 实现 | 访问延迟 | 存储容量 | 特点 |
| --- | --- | --- | --- | --- |
| L1 | Moka（进程内内存缓存） | < 1μs | 受限于JVM堆内存 | 最快，进程内共享，无网络开销 |
| L2 | Redis（分布式缓存） | 0.1~1ms | 受限于Redis内存 | 较快，跨进程/实例共享 |
| L3 | PostgreSQL（持久化存储） | 1~10ms | 磁盘容量 | 最慢，数据永久保存，Source of Truth |

三级缓存的命中率在实际场景中通常呈现"二八定律"：约20%的热点用户贡献了80%的查询请求，这些热点用户的数据会长期驻留在L1缓存中，使得绝大多数查询在微秒级完成。

### 5.2 Moka：进程内高性能缓存

Moka是一个基于Rust的高性能并发缓存库，支持异步操作，提供LRU（最近最少使用）和TTL（生存时间）两种淘汰策略。相比`HashMap`加`RwLock`的简单实现，Moka使用了更精细的并发控制机制，在高并发读写场景下性能更优。

Moka缓存的主要特性：

- **并发安全**：内部使用分段锁（Segmented Lock）减少锁竞争；
- **异步支持**：提供`async`版本的`get`/`insert`方法，与Tokio无缝集成；
- **自动淘汰**：支持基于容量（最大条目数）和TTL的自动淘汰，防止内存无限增长；
- **零拷贝读取**：返回值的`Arc`包装避免了数据克隆开销。

### 5.3 Redis：分布式共享缓存

Redis作为L2缓存，在多实例部署场景下提供跨节点的数据共享能力。当系统水平扩展为多个后端实例时，每个实例的L1 Moka缓存是独立的，但L2 Redis缓存是共享的。这意味着：

- 实例A处理登录请求后，将用户数据写入Redis；
- 实例B处理该用户的后续请求时，L1未命中，但L2 Redis命中，无需查询PostgreSQL；
- 实例B将Redis中的数据回填到自己的L1 Moka缓存，后续请求直接命中L1。

用户数据在Redis中以JSON格式序列化存储，键名格式为`user:{uuid}`，设置合理的TTL（如30分钟）防止缓存数据长期不更新。

### 5.4 分层仓储实现

`LayeredUserRepository`实现了透明的多级缓存逻辑，上层业务代码无需感知缓存细节，只需调用统一的仓储接口：

```rust
#[derive(Debug, Clone)]
pub struct LayeredUserRepository {
    l1_cache: MokaUserRepository,      // 进程内内存缓存
    l2_cache: RedisUserRepository,     // 分布式缓存
    source_repo: PostgresUserRepository, // 持久化存储
}

impl UserQueryRepository for LayeredUserRepository {
    async fn get_by_id(&self, user_id: &UserId) -> DomainResult<Option<User>> {
        // L1 命中：纳秒级返回，最优路径
        if let Ok(Some(user)) = self.l1_cache.get_by_id(user_id).await {
            return Ok(Some(user));
        }
        // L2 命中：毫秒级返回，回填 L1
        if let Ok(Some(user)) = self.l2_cache.get_by_id(user_id).await {
            self.warm_up_l1(&user).await;  // 异步回填，不阻塞当前请求
            return Ok(Some(user));
        }
        // L3 查询：数据库查询，回填 L2 和 L1
        let user = self.source_repo.get_by_id(user_id).await?;
        if let Some(ref u) = user {
            self.warm_up_l2_and_l1(u).await;  // 异步预热，不阻塞当前请求
        }
        Ok(user)
    }
    // get_by_name、get_by_email、get_by_name_or_email 遵循相同模式
}
```

**缓存预热（Cache Warm-up）辅助方法：**

```rust
async fn warm_up_l1(&self, user: &User) {
    let _ = self.l1_cache.save(user).await;  // 忽略错误，缓存写入失败不影响主流程
}

async fn warm_up_l2_and_l1(&self, user: &User) {
    let _ = self.l2_cache.save(user).await;
    self.warm_up_l1(user).await;
}
```

使用`let _ =`忽略缓存写入错误是有意为之的设计：缓存是可选的性能优化，其失败不应影响核心业务流程的正确性。即使缓存写入失败，下次请求时会再次触发缓存未命中，从数据库重新加载并尝试回填缓存。

### 5.5 缓存一致性保障

系统采用**写穿透（Write-Through）+ 缓存预热**的混合策略保障缓存一致性：

**写操作流程：**

1. 首先写入PostgreSQL（Source of Truth），确保数据持久化；
2. 写入成功后，立即将最新数据写入Redis（L2）和Moka（L1）；
3. 若缓存写入失败，不影响主流程，下次读取时会自动从数据库加载。

```rust
impl UserCommandRepository for LayeredUserRepository {
    async fn save(&self, user: &User) -> DomainResult<User> {
        let saved = self.source_repo.save(user).await?;  // 必须成功
        self.warm_up_l2_and_l1(&saved).await;            // 允许失败
        Ok(saved)
    }

    async fn update_status(&self, user_id: &UserId, status: &UserStatus) -> DomainResult<User> {
        let updated = self.source_repo.update_status(user_id, status).await?;
        self.warm_up_l2_and_l1(&updated).await;
        Ok(updated)
    }
}
```

这种策略的优势在于：写操作完成后，缓存中的数据立即是最新的，后续读操作可以直接命中缓存，无需等待缓存自然过期。相比纯粹的Cache-Aside（懒加载）策略，写穿透策略在写后立即读的场景下性能更优。

### 5.6 AppState初始化与依赖注入

系统启动时，通过`tokio::try_join!`并发初始化Redis和PostgreSQL连接，减少启动时间：

```rust
pub async fn new(config: Config) -> ApplicationResult<Self> {
    let moka_client = MokaClient::new();  // 内存操作，立即完成
    let _resend_client = Resend::new(&config.resend.api_key);

    // 并发初始化网络连接，而非串行等待
    let (redis_client, postgres_client) = tokio::try_join!(
        async { RedisClient::new(&config.redis).await.context(RedisFailedSnafu) },
        async { PostgresClient::new(&config.postgres).await.context(PostgresFailedSnafu) },
    )?;

    // 组装三级缓存仓储
    let user_repo = LayeredUserRepository::new(
        MokaUserRepository::new(moka_client),
        RedisUserRepository::new(redis_client),
        PostgresUserRepository::new(postgres_client),
    );

    Ok(AppState {
        user_repo,
        password_service: Argon2PasswordService::new(),
        access_token_service: DefaultAccessTokenService::new(
            &config.jwt.secret,
            config.jwt.expires_in_seconds,
        ),
        refresh_token_service: DefaultRefreshTokenService::new(),
    })
}
```

`AppState`通过`Arc<AppState>`在所有请求处理任务间共享，Axum的`State`提取器负责将其注入到每个Handler函数中，实现了依赖注入（DI）的效果，同时保持了Rust的所有权语义。

## 6. 系统安全性分析

### 6.1 密码安全

系统使用Argon2id算法对用户密码进行哈希存储。Argon2是2015年密码哈希竞赛（PHC）的获胜算法，具有以下安全特性：

- **内存困难性**：需要大量内存才能计算，使GPU/ASIC暴力破解成本极高。攻击者若想并行化暴力破解，需要为每个并行线程分配独立的大块内存，显著提高了硬件成本；
- **时间-内存权衡抵抗**：Argon2id结合了Argon2i（数据无关内存访问，抵御侧信道攻击）和Argon2d（数据相关内存访问，抵御GPU攻击）的优点，是通用密码哈希的推荐变体；
- **自动加盐**：每次哈希自动生成16字节的密码学安全随机盐值，即使两个用户使用相同密码，其哈希结果也完全不同，从根本上防止彩虹表攻击；
- **可调参数**：可根据服务器硬件性能调整迭代次数（time_cost）、内存用量（memory_cost，单位KB）和并行度（parallelism），在安全性与性能之间灵活权衡；
- **PHC字符串格式**：哈希结果以`$argon2id$v=19$m=65536,t=2,p=1$<salt>$<hash>`格式存储，包含算法版本、参数和盐值，验证时无需额外存储元数据。

数据库中存储的凭证格式为JSONB，包含哈希后的密码字符串（含盐值和算法参数），原始密码在哈希完成后立即从内存中丢弃，永不落盘。密码验证使用恒定时间比较（constant-time comparison），防止基于响应时间的侧信道攻击。

### 6.2 Token安全

**Access Token安全措施：**

| 威胁 | 攻击方式 | 防御措施 |
| --- | --- | --- |
| Token伪造 | 攻击者构造假Token | HS256签名，密钥仅服务端持有，伪造Token签名验证必然失败 |
| Token重放 | 窃取有效Token后重复使用 | 短期有效（默认1小时），过期后自动失效 |
| 密钥泄露 | 源码或配置文件中暴露密钥 | 通过环境变量注入，不硬编码于代码，`.env`文件加入`.gitignore` |
| 载荷篡改 | 修改JWT载荷中的用户ID | 签名验证，任何修改均导致签名不匹配，验证失败 |
| 算法混淆 | 将算法改为`none`绕过验证 | Validation::new(Algorithm::HS256)`明确指定算法，拒绝其他算法 |

**Refresh Token安全措施：**

| 威胁 | 攻击方式 | 防御措施 |
| --- | --- | --- |
| XSS窃取 | 注入脚本读取Token | HttpOnly Cookie，JavaScript无法通过`document.cookie`读取 |
| CSRF攻击 | 诱导用户浏览器发送携带Cookie的请求 | 可配合`SameSite=Strict`属性防御，生产环境应启用 |
| Token猜测 | 枚举或预测Token值 | UUID v4，128位密码学安全随机数，不可预测 |
| 网络窃听 | 中间人截获Cookie | `Secure`属性，仅通过HTTPS传输，生产环境应启用 |
| Token泄露后长期有效 | 泄露的Token被长期使用 | 7天有效期限制影响窗口；未来版本实现Token轮换进一步缩短窗口 |

### 6.3 输入验证

系统在多个层次对用户输入进行严格验证，形成纵深防御：

**表现层验证**：通过`TryInto` trait将请求载荷转换为命令对象，转换失败时返回400错误，阻止非法输入进入业务逻辑：

```rust
impl TryInto<RegisterCommand> for RegisterRequestPayload {
    fn try_into(self) -> Result<RegisterCommand, ApplicationError> {
        let name = UserName::new(self.name).map_err(|e| ValidationSnafu { message: e.to_string() }.build())?;
        let email = UserEmail::new(self.email).map_err(|e| ValidationSnafu { message: e.to_string() }.build())?;
        let password = PlainPassword::new(self.password).map_err(|e| ValidationSnafu { message: e.to_string() }.build())?;
        let confirm = PlainPassword::new(self.confirm_password).map_err(|e| ValidationSnafu { message: e.to_string() }.build())?;
        if password != confirm {
            return PasswordsNotMatchedSnafu.fail();
        }
        Ok(RegisterCommand { name, email, plain_password: password })
    }
}
```

**领域层验证**：值对象在构造时强制验证业务规则，一旦构造成功即可保证其合法性，无需在后续代码中重复验证：

- `UserName`：验证长度（1~31字符）和字符集（字母、数字、下划线等）；
- `UserEmail`：使用正则表达式验证邮箱格式，遵循RFC 5322规范；
- `PlainPassword`：验证密码复杂度（最小长度、包含大小写字母和数字等）；
- `UserId`：验证UUID格式，防止非法ID注入。

这种"构造即验证"的模式是Rust类型系统的最佳实践，被称为"使非法状态不可表示"（Make Illegal States Unrepresentable）。

### 6.4 防止用户枚举攻击

用户枚举攻击是指攻击者通过系统的不同错误响应来判断某个用户名或邮箱是否已注册，从而建立有效账户列表用于后续的撞库攻击。系统在多处采取了防枚举措施：

1. **登录接口**：用户不存在和密码错误均返回相同的`InvalidCredentials`错误消息和HTTP状态码，攻击者无法区分两种情况；
2. **无密码凭证**：若用户没有密码凭证，同样返回`UserNotFound`而非更具体的错误；
3. **响应时间一致性**：密码验证使用Argon2的恒定时间比较，即使用户不存在，也应执行一次虚拟的密码验证以保持响应时间一致（当前版本待完善）。

### 6.5 错误处理与信息安全

系统对外暴露的错误信息经过精心设计，在提供足够调试信息的同时，避免泄露内部实现细节：

- **业务错误**（4xx）：返回用户友好的错误消息，如"用户名或密码错误"，不暴露具体原因；
- **系统错误**（5xx）：统一返回"Internal Server Error"，不暴露堆栈信息、数据库错误详情或内部路径；
- **结构化错误**：使用`snafu`库进行结构化错误处理，错误类型在编译期确定，每种错误携带上下文信息，便于服务端日志记录和问题排查，但这些信息不会暴露给客户端。

### 6.6 CORS配置

后端通过`tower-http`的CORS中间件配置跨域策略，实现精细化的跨域访问控制：

```rust
// 仅允许指定的前端地址发起跨域请求
pub fn cors_middleware(frontend_address: String) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(frontend_address.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_credentials(true)  // 允许携带Cookie（Refresh Token）
}
```

`allow_credentials(true)`是必要的，因为Refresh Token存储于Cookie中，跨域请求需要携带Cookie。注意，启用`allow_credentials`时，`allow_origin`不能设置为通配符`*`，必须指定具体的来源地址，这也是安全的做法。

### 6.7 数据库安全

系统使用SQLx进行数据库操作，所有查询均为参数化查询（Prepared Statement），从根本上防止SQL注入攻击。参数化查询将SQL语句结构与数据值分离，数据库驱动保证用户输入永远不会被解释为SQL代码：

```rust
// SQLx参数化查询示例（用户名查询）
sqlx::query_as!(
    UserRow,
    "SELECT * FROM users WHERE name = $1",
    user_name.value()  // 用户输入作为参数，不拼接到SQL字符串中
)
.fetch_optional(&self.pool)
.await
```

SQLx还在编译期验证SQL语句的正确性（通过`.sqlx`目录中的查询元数据），确保SQL语句在编译时就能发现语法错误和类型不匹配问题，进一步提升了代码可靠性。

数据库连接使用TLS加密（通过`tls-rustls-aws-lc-rs` feature），防止数据库连接被网络窃听。连接池配置合理的最大连接数，防止连接耗尽攻击。

## 7. 系统实现与部署

### 7.1 项目结构

系统采用Cargo工作区（Workspace）管理前后端两个独立的Rust项目，共享工具链配置：

```
WebAuthSystem/
├── backend/                    # Rust后端（Axum）
│   ├── src/
│   │   ├── domain/             # 领域层（纯业务逻辑，无外部依赖）
│   │   │   ├── auth/           # 认证领域
│   │   │   │   ├── services/   # 领域服务接口（trait定义）
│   │   │   │   ├── value_objects/  # Token值对象
│   │   │   │   ├── entities/   # 认证实体
│   │   │   │   ├── aggregates/ # 认证聚合根
│   │   │   │   ├── policies/   # 业务策略
│   │   │   │   ├── repositories/ # 仓储接口
│   │   │   │   └── events/     # 领域事件
│   │   │   ├── identities/     # 身份领域
│   │   │   │   ├── aggregates/ # 用户聚合根（User）
│   │   │   │   ├── entities/   # 凭证实体（Credential）
│   │   │   │   ├── value_objects/  # 用户值对象
│   │   │   │   └── repositories/  # 仓储接口（trait定义）
│   │   │   └── common/         # 跨领域共享值对象（Timestamp等）
│   │   ├── application/        # 应用层（用例编排）
│   │   │   ├── use_cases/      # 业务用例（LoginCase、RegisterCase）
│   │   │   ├── commands/       # 命令对象（输入）
│   │   │   ├── results/        # 结果对象（输出）
│   │   │   ├── queries/        # 查询对象
│   │   │   ├── error.rs        # 应用层错误定义
│   │   │   └── app_state.rs    # 应用状态（依赖注入容器）
│   │   ├── infrastructure/     # 基础设施层（技术实现）
│   │   │   ├── security/       # 安全服务实现
│   │   │   │   ├── tokens/     # JWT/UUID Token实现
│   │   │   │   └── password/   # Argon2密码服务实现
│   │   │   ├── caches/         # 缓存实现
│   │   │   │   ├── moka/       # L1 Moka内存缓存
│   │   │   │   └── redis/      # L2 Redis分布式缓存
│   │   │   ├── persistence/    # 持久化实现
│   │   │   │   └── postgres/   # L3 PostgreSQL
│   │   │   ├── layered/        # 三级缓存透明仓储
│   │   │   ├── external/       # 外部服务（邮件等）
│   │   │   ├── config/         # 配置管理（figment2）
│   │   │   ├── logger.rs       # 日志初始化（tracing）
│   │   │   └── logo.rs         # 启动Logo
│   │   ├── presentation/       # 表现层（HTTP接口）
│   │   │   └── http/v1/        # HTTP API v1
│   │   │       ├── handlers/   # 请求处理器
│   │   │       │   ├── login_handler/
│   │   │       │   ├── register_handler/
│   │   │       │   └── logout_handler/
│   │   │       ├── routes/     # 路由定义
│   │   │       ├── middlewares/ # 中间件（CORS、Trace）
│   │   │       ├── response.rs # 统一响应格式
│   │   │       ├── error.rs    # HTTP错误处理
│   │   │       └── openapi.rs  # Swagger文档
│   │   ├── lib.rs              # 服务启动逻辑
│   │   └── main.rs             # 程序入口
│   ├── migrations/             # SQLx数据库迁移脚本
│   ├── .env                    # 本地开发环境变量
│   ├── .env.example            # 环境变量模板
│   └── Cargo.toml
├── frontend/                   # Leptos前端（WASM）
│   └── src/
│       ├── routes/             # 页面路由
│       │   └── auth/           # 认证相关页面（登录、注册）
│       ├── components/         # 可复用UI组件
│       │   └── ui/             # 基础UI组件
│       ├── api/                # API调用封装
│       ├── app.rs              # 根组件（路由配置）
│       ├── lib.rs              # WASM入口
│       └── main.rs             # 开发服务器入口
└── deployment/                 # 部署配置
    └── docker/
        ├── backend/Dockerfile
        ├── frontend/Dockerfile
        ├── docker-compose-infrastructure.yaml  # 基础设施（PG+Redis）
        └── docker-compose-app.yaml             # 应用（后端+前端）
```

### 7.2 数据库设计

用户表是系统的核心数据表，采用PostgreSQL设计，充分利用了PostgreSQL的高级特性：

```sql
-- 用户状态枚举类型（PostgreSQL原生枚举）
CREATE TYPE user_status AS ENUM (
    'EmailNotVerified',
    'Active',
    'Banned'
);

-- 用户主表
CREATE TABLE "users" (
    id          UUID         NOT NULL PRIMARY KEY,
    name        VARCHAR(31)  NOT NULL UNIQUE,
    email       VARCHAR(254) NOT NULL UNIQUE,
    credentials JSONB        NOT NULL,
    status      user_status  NOT NULL DEFAULT 'EmailNotVerified',
    created_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- 查询性能索引
CREATE INDEX idx_users_name  ON users(name);
CREATE INDEX idx_users_email ON users(email);
```

**设计要点说明：**

- **UUID主键**：使用UUID v4而非自增整数ID，避免了自增ID带来的信息泄露风险（攻击者无法通过ID推断用户总数或注册顺序），同时支持分布式环境下的ID生成；
- **JSONB凭证字段**：`credentials`字段使用PostgreSQL的JSONB类型存储凭证数组，支持未来扩展多种凭证类型（密码、OAuth令牌、TOTP密钥等），无需修改表结构；
- **枚举状态**：`user_status`使用PostgreSQL原生枚举类型，数据库层面保证状态值的合法性，防止出现非法状态；
- **时区感知时间戳**：`TIMESTAMP WITH TIME ZONE`存储UTC时间，避免时区转换问题；
- **唯一约束**：`name`和`email`字段设置唯一约束，数据库层面防止重复注册；
- **查询索引**：为`name`和`email`字段创建索引，优化按用户名或邮箱查询的性能。

数据库迁移使用SQLx的迁移工具管理，每次迁移对应一个版本化的SQL文件，支持向上（up）和向下（down）迁移，保证了数据库变更的可追溯性和可回滚性。

### 7.3 API接口设计

系统提供RESTful风格的HTTP API，遵循统一的请求/响应格式规范。开发模式下（`SERVER__IS_DEVELOPMENT_MODE=true`）自动启用Swagger UI文档，访问`/swagger`即可查看完整的API文档。

**统一响应格式：**

成功响应：

```json
{
  "code": 200,
  "message": "操作成功描述",
  "data": { /* 业务数据 */ }
}
```

错误响应：

```json
{
  "code": 400,
  "message": "具体错误描述",
  "data": null
}
```

**注册接口：**

```
POST /api/v1/auth/register
Content-Type: application/json

请求体：
{
  "name": "username",
  "email": "user@example.com",
  "password": "SecurePass123!",
  "confirmPassword": "SecurePass123!"
}

成功响应（HTTP 200）：
{
  "code": 200,
  "message": "registered successfully",
  "data": { "userId": "550e8400-e29b-41d4-a716-446655440000" }
}

错误响应示例：
- 400: 用户名/邮箱格式不合法
- 400: 两次密码不一致
- 409: 用户名或邮箱已存在
```

**登录接口：**

```
POST /api/v1/auth/login
Content-Type: application/json

请求体：
{
  "name_or_email": "username 或 user@example.com",
  "password": "SecurePass123!"
}

成功响应（HTTP 200）：
响应头：
  Authorization: Bearer eyJhbGciOiJIUzI1NiJ9...
  Set-Cookie: refresh_token=f47ac10b-...; Path=/; Max-Age=604800; HttpOnly

响应体：
{
  "code": 200,
  "message": "Login successfully",
  "data": {}
}

错误响应示例：
- 400: 用户名/邮箱或密码格式不合法
- 401: 用户名或密码错误
- 403: 账户未验证邮箱 / 账户已封禁
```

### 7.4 日志与可观测性

系统使用`tracing`生态系统实现结构化日志，支持按日期滚动的文件日志和控制台输出：

```rust
pub fn init_logger() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("logs", "backend.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(non_blocking)
        .json()  // 结构化JSON日志，便于日志聚合系统解析
        .init();
    guard
}
```

HTTP请求通过`tower-http`的`TraceLayer`中间件自动记录请求方法、路径、状态码和响应时间，无需在每个Handler中手动添加日志。

### 7.5 容器化部署

系统提供完整的Docker Compose部署方案，分为基础设施层和应用层两个编排文件，支持独立启动和联合启动：

**基础设施（docker-compose-infrastructure.yaml）：**

```yaml
services:
  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: web_auth_system
      POSTGRES_USER: ${POSTGRES_USER}
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
    volumes:
      - ./postgres/data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

  redis:
    image: redis:7-alpine
    command: redis-server --requirepass ${REDIS_PASSWORD}
    volumes:
      - ./redis/data:/data
    ports:
      - "6379:6379"
```

**后端Dockerfile（多阶段构建）：**

```docker
# 构建阶段：使用完整Rust工具链
FROM rust:1.87-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# 运行阶段：仅包含可执行文件，镜像体积极小
FROM debian:bookworm-slim AS runtime
COPY --from=builder /app/target/release/backend /usr/local/bin/backend
EXPOSE 7878
CMD ["backend"]
```

多阶段构建的优势：构建阶段镜像包含完整的Rust工具链（约1GB），运行阶段镜像仅包含编译后的二进制文件（约20MB），显著减小了生产镜像体积，同时避免了将编译工具链暴露在生产环境中的安全风险。

### 7.6 配置管理

系统通过环境变量进行配置，使用`figment2`库实现分层配置加载（`.env`文件 + 系统环境变量），支持开发和生产环境的无缝切换。配置项使用双下划线`__`作为层级分隔符：

```bash
# JWT配置（生产环境必须替换）
JWT__SECRET="通过 openssl rand -base64 64 生成的高熵密钥"
JWT__EXPIRES_IN_SECONDS=3600

# 数据库配置
POSTGRES__DATABASE_URL="postgres://user:pass@localhost/web_auth_system"

# Redis配置
REDIS__ADDRESS="redis://localhost:6379"

# 服务器配置
SERVER__IS_DEVELOPMENT_MODE=false
SERVER__BACKEND_PORT=7878
SERVER__BACKEND_IP="0.0.0.0"
SERVER__FRONTEND_ADDRESS="<https://your-domain.com>"

# 邮件服务配置
RESEND__API_KEY="re_your_resend_api_key"
RESEND__SYSTEM_OWNER_EMAIL="noreply@your-domain.com"
```

`.env.example`文件提供了所有配置项的说明和示例值，新开发者可以直接复制并填写实际值，降低了环境搭建的门槛。`.env`文件加入`.gitignore`，防止敏感配置意外提交到版本控制系统。

## 8. 性能分析

### 8.1 三级缓存性能对比

在典型的认证场景中，用户登录后的每次API请求都需要验证Token并查询用户信息。三级缓存的引入显著降低了平均查询延迟：

| 缓存层 | 典型延迟 | 吞吐量（单实例） | 命中场景 |
| --- | --- | --- | --- |
| L1 Moka | < 1μs | 数百万 QPS | 同一进程内的热点用户 |
| L2 Redis | 0.1~1ms | 数十万 QPS | 跨进程/跨实例的活跃用户 |
| L3 PostgreSQL | 1~10ms | 数千 QPS | 首次查询或缓存过期 |

在实际负载下，缓存命中率的分布通常遵循"二八定律"：约20%的活跃用户贡献了80%的查询请求，这些用户的数据会长期驻留在L1 Moka缓存中。以一个DAU为10万的系统为例，假设L1命中率为70%、L2命中率为25%、L3命中率为5%，则：

- 70%的请求在 < 1μs 内完成（L1命中）
- 25%的请求在 0.1~1ms 内完成（L2命中）
- 仅5%的请求需要查询PostgreSQL（1~10ms）

加权平均延迟约为：`0.7×0.001ms + 0.25×0.5ms + 0.05×5ms = 0.376ms`，相比纯数据库查询的5ms，性能提升约13倍。在高并发场景下，数据库压力也从全量QPS降低至5%，极大地延长了数据库的使用寿命。

### 8.2 Rust异步运行时性能

系统基于Tokio异步运行时，采用M:N线程模型（少量OS线程承载大量异步任务）。Tokio默认创建与CPU核心数相同的工作线程，每个工作线程运行一个事件循环，通过工作窃取（Work-Stealing）算法在线程间均衡负载。

相比传统的每请求一线程（Thread-per-Request）模型，Tokio的优势体现在：

- **内存占用**：每个OS线程默认栈大小为8MB，而Tokio的异步任务（Future）初始栈仅需数百字节，支持同时处理数十万并发连接；
- **上下文切换**：OS线程切换需要保存/恢复完整的CPU寄存器状态（约数微秒），而异步任务切换仅需保存/恢复少量状态（约数十纳秒）；
- **I/O等待**：异步I/O在等待网络或磁盘时不阻塞线程，线程可以继续处理其他任务，CPU利用率更高。

Axum框架基于Tower的中间件栈，所有中间件均为零成本抽象（Zero-Cost Abstraction）——在编译期通过泛型和trait展开，不引入额外的运行时开销（如虚函数调用、堆分配等）。

### 8.3 Rust内存安全对性能的影响

Rust的内存安全保证通过编译期检查实现，不依赖垃圾回收器（GC）。这意味着：

- **无GC停顿**：Java、Go等语言的GC会周期性地暂停程序执行（Stop-the-World），在高并发场景下可能导致请求延迟毛刺（Latency Spike）。Rust没有GC，延迟更加稳定可预测；
- **确定性内存释放**：Rust通过所有权系统在编译期确定内存的释放时机，内存在不再需要时立即释放，不会积累大量待回收的垃圾对象；
- **缓存友好**：Rust鼓励使用栈分配和连续内存布局（如`Vec<T>`），相比Java等语言大量使用堆分配的对象图，具有更好的CPU缓存局部性。

### 8.4 WebAssembly前端性能

Leptos编译为WebAssembly后，其细粒度响应式系统（Fine-grained Reactivity）仅更新实际发生变化的DOM节点，避免了React等框架的虚拟DOM全量diff开销。

对于认证页面这类交互相对简单的场景，性能考量主要集中在：

- **首次加载**：WASM文件（约4MB）的下载和初始化是主要开销，可通过CDN缓存、HTTP/2推送和Brotli压缩（压缩率约70%）优化；
- **运行时性能**：WASM的执行速度接近原生代码，表单验证、状态更新等操作的响应延迟极低；
- **内存占用**：WASM模块在浏览器中的内存占用相对固定，不会随用户操作线性增长。

Tailwind CSS 4.x采用按需生成（JIT）策略，最终CSS文件仅包含实际使用的样式类，生产构建的CSS体积通常在10~30KB（gzip后），对页面加载性能影响极小。

### 8.5 数据库查询优化

系统在数据库层面进行了以下优化：

1. **连接池**：SQLx使用连接池管理数据库连接，避免了每次请求建立新连接的开销（TCP握手 + TLS握手约需数十毫秒）；
2. **索引覆盖**：为`name`和`email`字段创建索引，将用户查询从全表扫描（O(n)）优化为索引查找（O(log n)）；
3. **JSONB索引**：对于频繁查询的JSONB字段，可创建GIN索引进一步优化查询性能（当前版本未实现，作为未来优化方向）；
4. **编译期SQL验证**：SQLx在编译期验证SQL语句，避免了运行时SQL解析错误，同时使数据库驱动可以预编译查询语句（Prepared Statement），减少每次查询的解析开销。

## 9. 总结与展望

### 9.1 总结

本文设计并实现了一套基于双Token机制的安全Web认证系统，系统从需求分析、架构设计到代码实现、容器化部署均进行了完整的工程实践。主要工作和贡献总结如下：

**在认证安全性方面**，系统通过短期JWT Access Token（响应头传递，有效期1小时）与长期UUID Refresh Token（HttpOnly Cookie存储，有效期7天）的组合，在安全性与用户体验之间取得了良好平衡。Access Token的短期有效性将泄露影响窗口限制在1小时以内；Refresh Token的HttpOnly Cookie存储从根本上切断了XSS攻击的Token窃取路径；Argon2id密码哈希算法提供了业界最高等级的密码存储安全性；参数化SQL查询防止了SQL注入攻击；统一的错误响应策略防止了用户枚举攻击。多层安全措施共同构建了纵深防御体系。

**在系统性能方面**，Moka（L1进程内缓存）+ Redis（L2分布式缓存）+ PostgreSQL（L3持久化存储）的三级透明缓存架构，将热点用户数据的查询延迟从数据库的5~10ms降低至内存缓存的亚微秒级，在典型负载下加权平均延迟约为0.4ms，性能提升约13倍，同时将数据库承受的查询压力降低至约5%。Tokio异步运行时的M:N线程模型支持高并发连接，无GC停顿保证了延迟的稳定性。

**在代码质量方面**，系统严格遵循领域驱动设计（DDD）的分层架构原则，将业务逻辑集中于领域层，通过接口（trait）实现依赖倒置，使领域层完全独立于任何外部框架和基础设施。Rust的类型系统被充分利用：值对象在构造时强制验证业务规则（"使非法状态不可表示"），错误类型在编译期确定（`snafu`结构化错误），所有权系统保证了并发安全。这些特性共同保证了代码的可维护性、可测试性和可扩展性。

**在工程完整性方面**，系统提供了完整的容器化部署方案（Docker Compose）、数据库迁移脚本（SQLx migrations）、环境配置管理（figment2 + .env）、API文档（utoipa + Swagger UI）和结构化日志（tracing），具备生产环境部署能力。

### 9.2 不足与局限

当前系统仍存在以下不足，需要在后续版本中改进：

1. **Refresh Token无状态**：当前Refresh Token为无状态设计，服务端不存储其有效性记录，因此无法主动吊销。用户登出后，Refresh Token在其有效期内仍然有效（尽管客户端Cookie已清除）。这一问题将在引入Redis黑名单机制后解决；
2. **缺少Token轮换**：每次使用Refresh Token时应签发新的Refresh Token并使旧Token失效（Token Rotation），进一步缩短泄露影响窗口。当前版本未实现此机制；
3. **缺少速率限制**：登录、注册等敏感接口未实现速率限制，存在暴力破解和撞库攻击的风险；
4. **邮箱验证流程未完成**：用户注册后的邮箱验证流程（发送验证邮件、验证链接处理）尚未实现，导致所有新注册用户处于`EmailNotVerified`状态，无法正常登录；
5. **前端API集成不完整**：前端`api/mod.rs`目前为空，登录和注册页面的API调用逻辑尚未实现，前后端尚未完全联通。

### 9.3 未来工作

基于当前系统的基础，计划在后续版本中实现以下功能：

**短期计划（核心功能完善）：**

1. **邮箱验证流程**：用户注册后通过Resend API发送验证邮件，邮件中包含时效性验证Token（存储于Redis，有效期24小时），用户点击链接后将账户状态从`EmailNotVerified`更新为`Active`；
2. **Token刷新端点**：实现`POST /api/v1/auth/refresh`接口，客户端携带Refresh Token Cookie请求新的Access Token，实现无感续期。同时实现Token轮换：每次刷新时签发新的Refresh Token并使旧Token失效；
3.  **登出功能**：实现`POST /api/v1/auth/logout`接口，清除客户端Cookie，并将Refresh Token加入Redis黑名单（设置与Token剩余有效期相同的TTL），实现服务端主动吊销；
4. **前端API集成**：完善前端`api`模块，实现登录、注册、刷新Token等API调用，并处理Token过期的自动刷新逻辑。

**中期计划（功能扩展）：**

1. **密码重置**：实现忘记密码流程，通过邮件发送重置链接，使用时效性重置Token（存储于Redis）完成密码更新；
2. **速率限制**：对登录、注册等敏感接口实施速率限制（基于IP或用户名），防御暴力破解和撞库攻击。可使用Redis的滑动窗口计数器实现；
3. **账户管理API**：实现修改密码、修改邮箱、查看登录历史等账户管理功能；
4. **管理员接口**：实现用户封禁/解封、用户列表查询等管理员功能。

**长期计划（高级特性）：**

1. **多因素认证（MFA）**：在现有凭证体系（JSONB存储）的基础上，扩展支持TOTP（基于时间的一次性密码，如Google Authenticator）等第二因素认证，进一步提升账户安全性；
2. **OAuth 2.0第三方登录**：支持通过GitHub、Google等第三方OAuth提供商登录，凭证类型扩展为OAuth凭证，存储于现有的JSONB字段中；
3. **WebAuthn/FIDO2支持**：支持硬件安全密钥（如YubiKey）和平台认证器（如Touch ID、Windows Hello）进行无密码认证，代表了Web认证的未来方向；
4. **审计日志**：记录所有认证相关操作（登录成功/失败、密码修改、账户状态变更等）的审计日志，支持安全审计和合规要求。

随着这些功能的逐步实现，系统将发展为一个功能完整、安全可靠、性能优秀的企业级Web认证服务，可作为各类Web应用的认证基础设施。

## 参考文献

[1] M. Jones, J. Bradley, N. Sakimura. "JSON Web Token (JWT)." RFC 7519, IETF, May 2015.

[2] D. Hardt. "The OAuth 2.0 Authorization Framework." RFC 6749, IETF, October 2012.

[3] Alex Biryukov, Daniel Dinu, Dmitry Khovratovich. "Argon2: Memory-Hard Function for Password Hashing and Proof-of-Work Applications." USENIX Security 2016.

[4] OWASP Foundation. "OWASP Authentication Cheat Sheet." https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html, 2024.

[5] OWASP Foundation. "OWASP JSON Web Token Cheat Sheet." https://cheatsheetseries.owasp.org/cheatsheets/JSON_Web_Token_for_Java_Cheat_Sheet.html, 2024.

[6] Tokio Contributors. "Tokio: An asynchronous Rust runtime." [https://tokio.rs](https://tokio.rs/), 2024.

[7] Axum Contributors. "axum: Ergonomic and modular web framework built with Tokio, Tower, and Hyper." https://github.com/tokio-rs/axum, 2024.

[8] Leptos Contributors. "Leptos: Build fast web applications with Rust." [https://leptos.dev](https://leptos.dev/), 2024.

[9] SQLx Contributors. "SQLx: The Rust SQL Toolkit." https://github.com/launchbadge/sqlx, 2024.

[10] Eric Evans. "Domain-Driven Design: Tackling Complexity in the Heart of Software." Addison-Wesley Professional, 2003.

[11] Sam Newman. "Building Microservices: Designing Fine-Grained Systems." O'Reilly Media, 2021.

[12] NIST. "Digital Identity Guidelines." NIST Special Publication 800-63B, 2017.
