-- retrieve the unpaid invoices for the given customer using a pipelined function

CREATE OR REPLACE TYPE invoice_row AS OBJECT
(
    id          NUMBER,
    contract_id NUMBER,
    status      NVARCHAR2(10),
    amount      NUMBER,
    issue_date  DATE,
    due_date    DATE
);

CREATE OR REPLACE TYPE invoice_table AS TABLE OF invoice_row;

CREATE OR REPLACE FUNCTION get_unpaid_invoices(p_customer_id IN NUMBER)
    RETURN invoice_table PIPELINED
AS
BEGIN
    FOR invoice_rec IN (SELECT i.*
                        FROM invoice i
                                 JOIN contract c ON i.CONTRACT_ID = c.ID
                        WHERE c.CUSTOMER_ID = p_customer_id
                          AND i.STATUS = 'UNPAID')
        LOOP
            PIPE ROW (invoice_row(invoice_rec.ID, invoice_rec.CONTRACT_ID, invoice_rec.STATUS, invoice_rec.AMOUNT,
                                  invoice_rec.ISSUE_DATE, invoice_rec.DUE_DATE));
        END LOOP;
    RETURN;
END get_unpaid_invoices;
