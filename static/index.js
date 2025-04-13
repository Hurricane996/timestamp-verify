var timeout = undefined

document.addEventListener("DOMContentLoaded", function() {
    document.getElementById("show_token").addEventListener("click", function() {
        fetch("/token").then(function(value) {
            value.json().then(function(object) {
                document.getElementById("token").value = object.token;
                document.getElementById("timestamp").value = object.timestamp;
                document.getElementById("time").value = object.time_formatted;


                navigator.clipboard.writeText(`Timestamp: ${object.timestamp}; Token: ${object.token}`)
                .then(function() {
                    document.getElementById("form_results").innerHTML = `
<div class="success">
    <h3>Token copied to clipboard</h3>
</div>`;
                })
                .catch(function(err) {
                    console.log(err)
                })
                
                if (timeout !== undefined) {
                    clearTimeout(timeout)
                }
                timeout = setTimeout(function() {
                    document.getElementById("token").value = ""
                    document.getElementById("timestamp").value = ""
                    document.getElementById("time").value = ""
                }, 60_000)
            })
        })
        .catch(function(err) {
            console.log(err)
        })
    })
})