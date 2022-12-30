CREATE TABLE `item` (
    `item_id` BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `product_id` BIGINT UNSIGNED NOT NULL,
    CONSTRAINT `fk_item_product`
        FOREIGN KEY (`product_id`) REFERENCES `product` (`product_id`)
)