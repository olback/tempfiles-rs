export class ParamsBuilder {

    constructor(obj) {
        if (typeof obj === 'object') {
            this.params = obj;
        } else {
            this.params = {}
        }
    }

    append(key, val) {
        this.params[key] = val;
    }

    delete(key) {
        delete this.params[key];
    }

    toString() {

        let str = '';

        for (let key in this.params) {

            if (str === '') {
                str += `?${key}=${encodeURIComponent(this.params[key])}`
            } else {
                str += `&${key}=${encodeURIComponent(this.params[key])}`
            }

        }

        return str;

    }

}
