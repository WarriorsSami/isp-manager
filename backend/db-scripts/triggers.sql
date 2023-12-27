-- update the amount for unpaid invoices when the associated subscription price is changed

CREATE OR REPLACE TRIGGER update_invoice_amount
    AFTER UPDATE OF price
    ON subscription
    FOR EACH ROW
BEGIN
    UPDATE invoice
    SET AMOUNT = :NEW.PRICE
    WHERE CONTRACT_ID IN (SELECT CONTRACT_ID
                          FROM contract
                          WHERE SUBSCRIPTION_ID = :NEW.ID)
      AND STATUS = 'UNPAID';
END;

-- update the invoice status when its amount is reached after inserting a series of payments

CREATE OR REPLACE TRIGGER update_invoice_status
    BEFORE INSERT
    ON payment
    FOR EACH ROW
DECLARE
    total_amount NUMBER;
    paid_amount  NUMBER;
BEGIN
    SELECT SUM(AMOUNT)
    INTO total_amount
    FROM invoice
    WHERE INVOICE.ID = :NEW.INVOICE_ID;

    SELECT SUM(AMOUNT)
    INTO paid_amount
    FROM payment
    WHERE INVOICE_ID = :NEW.INVOICE_ID;

    IF paid_amount + :NEW.AMOUNT >= total_amount THEN
        UPDATE invoice
        SET STATUS = 'PAID'
        WHERE ID = :NEW.INVOICE_ID;
    END IF;
END;
