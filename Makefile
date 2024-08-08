include .env
# database_url="mysql://root:123456@127.0.0.1:3306/admin?ssl-mode=disabled"

lint:
	@./.pre-commit

seaql-entity:
	@sea-orm-cli generate entity \
    	-u ${DATABASE_URL} \
		--with-serde both \
		--model-extra-derives 'utoipa::ToSchema' \
    	-o crates/entity/src/model

local:
	@env APP_DEPLOY_MODE=local cargo run -p app

.PHONY: 
	local