css:
	npx tailwindcss -i ./presentation/css/main.css -o ./public/app.css --minify

dev:
	APP_ENV=development cargo watch -x run -q --
