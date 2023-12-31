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

-- retrieve the contracts for the given customer using a pipelined function

CREATE OR REPLACE TYPE contract_row AS OBJECT
(
    id          NUMBER,
    customer_id NUMBER,
    subscription_id NUMBER,
    start_date  DATE,
    end_date    DATE
);

CREATE OR REPLACE TYPE contract_table AS TABLE OF contract_row;

CREATE OR REPLACE FUNCTION get_contracts(p_customer_id IN NUMBER)
    RETURN contract_table PIPELINED
AS
BEGIN
    FOR contract_rec IN (SELECT *
                         FROM contract
                         WHERE CUSTOMER_ID = p_customer_id)
        LOOP
            PIPE ROW (contract_row(contract_rec.ID, contract_rec.CUSTOMER_ID, contract_rec.SUBSCRIPTION_ID,
                                   contract_rec.START_DATE, contract_rec.END_DATE));
        END LOOP;
    RETURN;
END get_contracts;

-- retrieve the invoices for the given contract using a pipelined function

CREATE OR REPLACE FUNCTION get_invoices(p_contract_id IN NUMBER)
    RETURN invoice_table PIPELINED
AS
BEGIN
    FOR invoice_rec IN (SELECT *
                        FROM invoice
                        WHERE CONTRACT_ID = p_contract_id)
        LOOP
            PIPE ROW (invoice_row(invoice_rec.ID, invoice_rec.CONTRACT_ID, invoice_rec.STATUS, invoice_rec.AMOUNT,
                                  invoice_rec.ISSUE_DATE, invoice_rec.DUE_DATE));
        END LOOP;
    RETURN;
END get_invoices;

-- retrieve the payments for the given invoice using a pipelined function

CREATE OR REPLACE TYPE payment_row AS OBJECT
(
    id          NUMBER,
    invoice_id  NUMBER,
    amount      NUMBER,
    payment_date  DATE
);

CREATE OR REPLACE TYPE payment_table AS TABLE OF payment_row;

CREATE OR REPLACE FUNCTION get_payments(p_invoice_id IN NUMBER)
    RETURN payment_table PIPELINED
AS
BEGIN
    FOR payment_rec IN (SELECT *
                        FROM payment
                        WHERE INVOICE_ID = p_invoice_id)
        LOOP
            PIPE ROW (payment_row(payment_rec.ID, payment_rec.INVOICE_ID, payment_rec.AMOUNT, payment_rec.PAYMENT_DATE));
        END LOOP;
    RETURN;
END get_payments;