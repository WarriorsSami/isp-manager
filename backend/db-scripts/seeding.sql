-- Insert data

INSERT INTO customer (name, fullname, address, phone, cnp)
VALUES ('John', 'John Doe', 'Some address', '0123456789', '1234567890123');

INSERT INTO subscription (description, type, traffic, price, extra_traffic_price)
VALUES ('Some description', 'MOBILE', 100, 10, 1);

INSERT INTO contract (customer_id, subscription_id, start_date, end_date)
VALUES ((SELECT id FROM customer WHERE name = 'John'),
        (SELECT id FROM subscription WHERE description = 'Some description'), TO_DATE('01-01-2019', 'DD-MM-YYYY'),
        TO_DATE('01-01-2020', 'DD-MM-YYYY'));

INSERT INTO invoice (contract_id, issue_date, due_date, amount)
VALUES ((SELECT id
                     FROM contract
                     WHERE id =
                           (SELECT id FROM contract WHERE customer_id = (SELECT id FROM customer WHERE name = 'John'))),
        TO_DATE('01-01-2019', 'DD-MM-YYYY'), TO_DATE('01-02-2019', 'DD-MM-YYYY'), 10);

INSERT INTO payment (invoice_id, payment_date, amount)
VALUES ((SELECT id
                     FROM invoice
                     WHERE id = (SELECT id
                                 FROM invoice
                                 WHERE contract_id = (SELECT id
                                                      FROM contract
                                                      WHERE customer_id = (SELECT id FROM customer WHERE name = 'John')))),
        TO_DATE('01-01-2019', 'DD-MM-YYYY'), 10);