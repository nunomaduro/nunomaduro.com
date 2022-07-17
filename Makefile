build:
	npm ci
	npm run build

dev:
	APP_ENV=development cargo watch -x run -q --
