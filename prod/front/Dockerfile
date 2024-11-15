
FROM guergeiro/pnpm:22-8 as base

WORKDIR /draft_together
COPY ./front .

RUN pnpm install
RUN pnpx nuxt build

FROM node:22-bookworm-slim
COPY --from=base /draft_together/.output /draft_together

CMD ["node", "/draft_together/server/index.mjs"]