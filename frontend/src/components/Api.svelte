<script></script>

<h1>Api</h1>

<main>

    <section>

        <h2>Download</h2>

        <div class="url">
            <span class="method">GET</span>
            <span class="url">/d/&lt;id&gt;/&lt;password&gt;</span>
        </div>

    </section>

    <section>

        <h2>Upload</h2>

        <div class="url">
            <span class="method">POST</span>
            <span class="url">/api/upload</span>
        </div>

        <div class="params">
            <div class="param">
                <span class="name">filename</span>
                <span class="datatype">string (optional)</span>
                <span class="description">Set a filename</span>
            </div>
            <div class="param">
                <span class="name">maxviews</span>
                <span class="datatype">integer (optional)</span>
                <span class="description">Maximum times a file can be viewed</span>
            </div>
        </div>

        <div class="headers">
            <div class="header">
                <span class="name">Content-Type</span>
                <span class="description">Set the content type</span>
            </div>
        </div>

        <div class="body">
            Body: Raw file data
        </div>

        <code class="response">
            <pre>
&#123;
    "status": 201,
    "id": "d5mSqMV5VzcCm4Jy",
    "password": "Sn86BrP99kghljLK9bavY1DLwjuvN8rL",
    "delete_password": "KX0mQpVJPyyI8WNld7OsgifOTOxc8AiE",
    "download_url": "..."
&#125;
            </pre>
        </code>

    </section>

        <section>

        <h2>Metadata</h2>

        <div class="url">
            <span class="method">GET</span>
            <span class="url">/api/metadata/&lt;id&gt;/&lt;password&gt;</span>
        </div>

        <code class="response">
            <pre>
&#123;
    "status": 200,
    "content_type": "audio/ogg",
    "content_length": 118118,
    "filename": "En-au-oof.ogg"
&#125;
            </pre>
        </code>

    </section>

    <section>

        <h2>Delete</h2>

        <div class="url">
            <span class="method">DELETE</span>
            <span class="url">/api/delete/&lt;id&gt;/&lt;deletion_password&gt;</span>
        </div>

        <code class="response">
            <pre>
&#123;
    "status": 200
&#125;
            </pre>
        </code>

    </section>

</main>

<style type="text/scss">

    @import 'src/scss/variables';

    h1 {
        color: $accent;
    }

    main {
        display: grid;
        row-gap: 2em;
    }

    section {

        color: #fff;
        font-size: 12pt;
        display: grid;
        row-gap: 1em;

        h2 {
            color: $accent;
            margin: 0;
        }

        div.url {

            span.method {
                background-color: $background2;
                padding: 1px 2px;
                margin-right: 2px;
                border: solid #ddd 1px;
                border-radius: 3px;
                font-family: monospace;
            }

            span.url {
                font-family: monospace;
            }

        }

        div.params, div.headers {

            display: grid;
            row-gap: .2em;

            &::before {
                margin-bottom: .5em;
                display: block;
                content: 'Params:';
                font-family: monospace;
            }

            div.param, div.header {

                span {
                    display: inline-block;
                    margin-right: .5em;
                }

                span.name {
                    background-color: $background;
                    padding: 1px 2px;
                    border: solid $background2 1px;
                    border-radius: 3px;
                    font-family: monospace;
                }

                span.datatype {
                    font-style: italic;
                    font-family: monospace;
                }

                // span.description {}

            }

        }

        div.headers::before {
            content: 'Headers:';
        }

        div.body {
            font-family: monospace;
        }

        code.response {

            font-family: monospace;

            &::before {
                margin-bottom: .5em;
                display: block;
                content: 'Response:';
            }

            pre {
                font-family: monospace;
                padding: 1em;
                margin: 0;
                background-color: #fff;
                color: $background;
            }

        }

    }

</style>
