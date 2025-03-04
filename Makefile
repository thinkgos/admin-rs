include .env

lint:
	@./.pre-commit

seaql-entity:
	@sea-orm-cli generate entity \
    	-u ${DATABASE_URL} \
		--with-serde both \
		--model-extra-derives 'utoipa::ToSchema' \
    	-o crates/entity/src/model

build.local.run:
	@env APP_DEPLOY_MODE=local cargo run -p app

.PHONY: 
	local