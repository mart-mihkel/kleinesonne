FROM node:22-alpine AS build
WORKDIR /usr/src/app

COPY . /usr/src/app/
RUN npm install --verbose --frozen-lockfile

ENV NODE_ENV=production
ENV ORIGIN=https://kleinesonne.ee
ENV PORT=5173
RUN npm run build

RUN rm -rf node_modules
RUN npm install --verbose --frozen-lockfile --production



FROM node:22-alpine
WORKDIR /usr/src/app

COPY --from=build /usr/src/app/node_modules node_modules
COPY --from=build /usr/src/app/package.json package.json
COPY --from=build /usr/src/app/build build

USER node
EXPOSE 5173
ENTRYPOINT [ "node", "build" ]
