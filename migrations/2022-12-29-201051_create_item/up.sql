CREATE TABLE "items" (
    "item_id" BIGSERIAL PRIMARY KEY ,
    "product_id" BIGSERIAL references "products"("product_id")
)