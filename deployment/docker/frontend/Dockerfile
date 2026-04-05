FROM m.daocloud.io/docker.io/library/node:22-alpine AS builder

WORKDIR /app

RUN npm install -g pnpm

COPY frontend/package.json frontend/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

COPY frontend/ .
RUN pnpm build

FROM m.daocloud.io/docker.io/library/node:22-alpine

WORKDIR /app

COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json .

EXPOSE 3000

CMD ["node", "build"]
