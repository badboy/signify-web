const rust = import("./signify_web");

rust.then(m => {
  document.getElementById("generate-new").onclick = function() {
    let key = m.generate_keypair();
    let [pub, secret] = key.split("|");

    document.getElementById("public-key").value = pub;
    document.getElementById("secret-key").value = secret;
  }

  document.getElementById("sign").onclick = function() {
    let message = document.getElementById("sign-message").value;
    let secret = document.getElementById("sign-secret-key").value;

    let value = m.sign(message, secret);
    document.getElementById("sign-signature").value = value;
  }

  document.getElementById("verify").onclick = function() {
    let message = document.getElementById("verify-message").value;
    let signature = document.getElementById("verify-signature").value;
    let secret = document.getElementById("verify-public-key").value;

    let out = document.getElementById("verify-result");
    if (m.verify(message, signature, secret)) {
      out.value = "Valid";
    } else {
      out.value = "Invalid";
    }
  }
})
