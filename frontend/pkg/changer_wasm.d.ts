/* tslint:disable */
/* eslint-disable */

export class Change {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

export class Transaction {
    free(): void;
    [Symbol.dispose](): void;
    constructor(cost: number, given: number);
}

export function calculate_change(data: Transaction): string;
