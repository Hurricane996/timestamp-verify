console.log("Hello, world!")


var timeout = undefined

document.addEventListener("DOMContentLoaded", function() {
    document.getElementById("show_token").addEventListener("click", function() {
        fetch("/token").then(function(value) {
            value.json().then(function(object) {
                document.getElementById("token").value = object.token;
                document.getElementById("timestamp").value = object.timestamp;
                document.getElementById("time").value = object.time_formatted;
                
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

        console.log("running");
    })
})