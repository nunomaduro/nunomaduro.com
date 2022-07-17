js:
	npm ci
	npm run build

css:
	npx tailwindcss -i ./presentation/css/main.css -o ./public/css/app.css --minify

dev:
	APP_ENV=development cargo watch -x run -q --
