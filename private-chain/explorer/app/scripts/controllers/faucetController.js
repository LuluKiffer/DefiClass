angular.module('ethExplorer')
    .controller('faucetCtrl', function ($rootScope, $scope, $location) {

	var form = document.getElementById('form');
	form.onsubmit = function(event) {
		const form = event.target;
		const email = form.elements['email'].value;
		const eth_addr = form.elements['eth_address'].value;
		var xhr = new XMLHttpRequest();
		var formData = new FormData(form);
		//xhr.open('POST', 'http://ec2-44-206-59-105.compute-1.amazonaws.com:9000/auth')
		xhr.open('POST', 'http://ec2-54-160-156-222.compute-1.amazonaws.com:9000/auth')
		xhr.setRequestHeader("Content-Type", "application/json");

		xhr.send(`{"email": "${email}", "eth_address": "${eth_addr}"}`);

		xhr.onreadystatechange = function() {
		    if (xhr.readyState == XMLHttpRequest.DONE) {
			var success = document.getElementById('success');
			var error = document.getElementById('error');
			    if (xhr.status === 200) {
				success.hidden = false;
				error.hidden = true;
				form.reset(); //reset form after AJAX success or do something else
			    } else {
				success.hidden = true;
				error.hidden = false;
				error.innerHTML = `ERROR: ${xhr.responseText}`;
			    }
		    }
		}
		return false;
	}

    });
