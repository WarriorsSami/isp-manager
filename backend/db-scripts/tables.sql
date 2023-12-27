-- Create tables

CREATE TABLE customer
(
    id       NUMBER GENERATED ALWAYS AS IDENTITY (START WITH 1 INCREMENT BY 1) PRIMARY KEY,
    name     NVARCHAR2(100) NOT NULL,
    fullname NVARCHAR2(100) NOT NULL,
    address  NVARCHAR2(100) NOT NULL,
    phone    NVARCHAR2(12)  NOT NULL,
    cnp      NVARCHAR2(13)  NOT NULL,

    CONSTRAINT valid_phone CHECK (REGEXP_LIKE(phone, '^[0-9]{10,12}$')),
    CONSTRAINT valid_cnp CHECK (REGEXP_LIKE(cnp, '^[0-9]{13}$'))
);

CREATE TABLE subscription
(
    id                  NUMBER GENERATED ALWAYS AS IDENTITY (START WITH 1 INCREMENT BY 1) PRIMARY KEY,
    description         NVARCHAR2(100) NOT NULL,
    type                NVARCHAR2(100) NOT NULL,
    traffic             NUMBER         NOT NULL,
    price               NUMBER         NOT NULL,
    extra_traffic_price NUMBER         NOT NULL,

    CONSTRAINT valid_type CHECK (type IN ('MOBILE', 'FIXED', 'TV', 'MOBILE_INTERNET', 'FIXED_INTERNET')),
    CONSTRAINT valid_traffic CHECK (traffic > 0),
    CONSTRAINT valid_price CHECK (price > 0),
    CONSTRAINT valid_extra_traffic_price CHECK (extra_traffic_price > 0)
);

CREATE TABLE contract
(
    id              NUMBER GENERATED ALWAYS AS IDENTITY (START WITH 1 INCREMENT BY 1) PRIMARY KEY,
    customer_id     NUMBER NOT NULL,
    subscription_id NUMBER NOT NULL,
    start_date      DATE    NOT NULL,
    end_date        DATE    NOT NULL,

    CONSTRAINT valid_start_date CHECK (start_date < end_date),
    CONSTRAINT fk_customer FOREIGN KEY (customer_id) REFERENCES customer (id),
    CONSTRAINT fk_subscription FOREIGN KEY (subscription_id) REFERENCES subscription (id)
);

CREATE TABLE invoice
(
    id          NUMBER GENERATED ALWAYS AS IDENTITY (START WITH 1 INCREMENT BY 1) PRIMARY KEY,
    contract_id NUMBER NOT NULL,
    issue_date  DATE    NOT NULL,
    due_date    DATE    NOT NULL,
    amount      NUMBER  NOT NULL,
    status      NVARCHAR2(10) DEFAULT 'unpaid' NOT NULL,

    CONSTRAINT valid_issue_date CHECK (issue_date < due_date),
    CONSTRAINT valid_amount CHECK (amount > 0),
    CONSTRAINT valid_status CHECK (status IN ('unpaid', 'paid')),
    CONSTRAINT fk_contract FOREIGN KEY (contract_id) REFERENCES contract (id)
);

ALTER TABLE invoice MODIFY status DEFAULT 'UNPAID';
ALTER TABLE invoice DROP CONSTRAINT valid_status;
ALTER TABLE invoice ADD CONSTRAINT valid_status CHECK (status IN ('UNPAID', 'PAID'));

CREATE TABLE payment
(
    id           NUMBER GENERATED ALWAYS AS IDENTITY (START WITH 1 INCREMENT BY 1) PRIMARY KEY,
    invoice_id   NUMBER NOT NULL,
    payment_date DATE    NOT NULL,
    amount       NUMBER  NOT NULL,

    CONSTRAINT valid_payment_amount CHECK (amount > 0),
    CONSTRAINT fk_invoice FOREIGN KEY (invoice_id) REFERENCES invoice (id)
);
