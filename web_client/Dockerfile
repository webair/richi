FROM node:24.7.0-alpine3.21 AS builder

WORKDIR /app
COPY ./package.json ./package.json
COPY ./package-lock.json ./package-lock.json
RUN npm ci
COPY . .
RUN npm run verify
RUN npm run build

FROM nginx:1.29.1-alpine
RUN cat << 'EOF' > /etc/nginx/conf.d/default.conf
server {
    listen      80;
    server_name localhost;
    location / {
        try_files $uri $uri/ /index.html;
        root    /usr/share/nginx/html;
        index  index.html index.htm;
    }
    error_page  500 502 503 504  /50x.html;
    location = /50x.html {
        root    /usr/share/nginx/html;
    }
}
EOF
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 80
