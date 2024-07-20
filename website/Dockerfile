FROM node:22-alpine AS build
WORKDIR /usr/src/app

COPY . .
RUN npm install -g vite --verbose
RUN npm install -g @rollup/plugin-commonjs --verbose
RUN npm install --verbose --frozen-lockfile

COPY package*.json /tmp/
RUN cd /tmp && npm install --production --verbose --frozen-lockfile

ENV NODE_ENV=production
ENV ORIGIN=https://localhost:443
ENV PORT=5173
RUN npm run build




FROM build AS prod

COPY --from=build /usr/src/app/package.json package.json 
COPY --from=build /tmp/node_modules node_modules
COPY --from=build /usr/src/app/build build

USER node
EXPOSE 5173
ENTRYPOINT node build
