FROM node:lts-alpine as base
WORKDIR /app
COPY . .

FROM base as test
RUN yarn
RUN ["npm", "run", "test"]

FROM base as production
RUN yarn install --production
CMD ["yarn", "start"]