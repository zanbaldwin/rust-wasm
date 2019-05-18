// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg/hello_world';`
// will work here one day as well!

class WasmApplication
{
    module: typeof import('../build/pkg/wasm');

    constructor(module: typeof import('../build/pkg/wasm'))
    {
        this.module = module;
    }

    run(): void
    {
        while (true) {
            let n = parseInt(prompt('Enter a number to find the nth prime number.') || '1');
            let start = performance.now();
            let primeNumber = this.getNthPrimeNumber(n);
            let ms = performance.now() - start;
            console.log(`Calculating prime number #${n} (${primeNumber}) took ${ms} milliseconds.`);
            alert(`The ${n} prime number is ${primeNumber}!`);
        }
    }

    getNthPrimeNumber(n: number): number
    {
        return this.module.nth_prime_number(n);
    }
}

export default WasmApplication;