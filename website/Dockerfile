FROM node:22-alpine AS install
WORKDIR /usr/src/app

COPY . .
RUN npm install -g vite --verbose
RUN npm install -g @rollup/plugin-commonjs --verbose
RUN npm install --verbose --frozen-lockfile
RUN npm run build

RUN mkdir -p /tmp/prod
COPY package*.json /tmp/prod/
RUN cd /tmp/prod && npm install --production --verbose --frozen-lockfile



FROM install AS prod
WORKDIR /usr/src/app

COPY --from=install /usr/src/app/build build
COPY --from=install /usr/src/app/package.json package.json 
COPY --from=install /tmp/prod/node_modules node_modules

USER node
EXPOSE 3000/tcp
ENTRYPOINT [ "node", "build" ]
