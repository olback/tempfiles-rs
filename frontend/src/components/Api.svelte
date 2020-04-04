<script></script>

<h1>Api</h1>

<section>

    <h2>Download</h2>
    <span class="method">GET</span>
    <span class="url">{window.origin}/d/&lt;id&gt;/&lt;password&gt;</span>

</section>

<section>

    <h2>Upload</h2>
    <span class="method">POST</span>
    <span class="url">{window.origin}/api/upload</span>

    <code class="response">
        <pre>
&#123;
    "status": 201,
    "id": "d5mSqMV5VzcCm4Jy",
    "password": "Sn86BrP99kghljLK9bavY1DLwjuvN8rL",
    "delete_password": "KX0mQpVJPyyI8WNld7OsgifOTOxc8AiE"
&#125;
        </pre>
    </code>

</section>

<section>

    <h2>Delete</h2>
    <span class="method">DELETE</span>
    <span class="url">{window.origin}/api/delete/&lt;id&gt;/&lt;deletion_password&gt;</span>

    <code class="response">
        <pre>
&#123;
    "status": 200
&#125;
        </pre>
    </code>

</section>

<style type="text/scss">

    @import 'src/scss/variables';

    h1 {
        color: $accent;
    }

    section {

        color: #fff;
        margin-bottom: 2em;
        font-size: 12pt;

        h2 {
            color: $accent;
        }

        span.method {
            background-color: $background2;
            padding: 1px 2px;
            margin-right: 2px;
            border: solid #ddd 1px;
            border-radius: 3px;
        }

        code {

            font-family: monospace;

            &.response::before {
                margin: 1em 0;
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
