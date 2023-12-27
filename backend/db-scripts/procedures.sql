-- retrieve the invoices for the given customer

CREATE OR REPLACE PROCEDURE get_invoices(p_customer_id IN RAW) AS
BEGIN
    SELECT i.*
    FROM invoice i
             JOIN contract c ON i.CONTRACT_ID = c.ID
    WHERE c.CUSTOMER_ID = p_customer_id;
END;