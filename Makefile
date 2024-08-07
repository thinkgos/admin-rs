# include .env
include temp/.env

lint:
	@./.pre-commit

seaql-entity:
	@sea-orm-cli generate entity \
    	-u ${database_url} \
		--with-serde both \
		--lib \
    	-o crates-entity/src

local:
	@env APP_DEPLOY_MODE=local APP_APP.PORT=9999 cargo run -p admin-app