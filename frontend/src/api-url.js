export class ApiUrl {

    /**
     * @param {string} baseUrl
     */
    constructor(baseUrl) {
        this.base = new URL(baseUrl);
    }

    /**
     * Get full url with path
     * @param {string} path
     * @returns {URL}
     */
    join(path) {

        return new URL(path, this.base);

    }

    /**
     * Get base API URL
     * @returns {URL} base API URL
     */
    getBase() {

        return this.base;

    }

}
