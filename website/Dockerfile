FROM node:22-alpine AS base
WORKDIR /usr/src/app



FROM base AS install

RUN mkdir -p /tmp/dev
COPY package*.json /tmp/dev/
RUN cd /tmp/dev && npm ci --frozen-lockfile --verbose

RUN mkdir -p /tmp/prod
COPY package*.json /tmp/prod/
RUN cd /tmp/prod && npm ci --frozen-lockfile --production --verbose



FROM base AS dev

COPY --from=install /tmp/dev/node_modules node_modules
COPY . .

ENV NODE_ENV=production
RUN npm run build



FROM base AS production

COPY --from=install /tmp/prod/node_modules node_modules
COPY --from=dev /usr/src/app/package.json package.json
COPY --from=dev /usr/src/app/build build

USER node
EXPOSE 3000/tcp
ENTRYPOINT [ "node", "build", "--trace-warnings" ]
