"use strict";

const application = Vue.createApp({
	mounted() {
		const video = document.querySelector("#video");
		const qrScanner = new QrScanner(video, result => this.updateQrCode(result));
		this.qrScanner = qrScanner;
	},

	data() {
		return {
			qrCode: "",
			loading: false,
			success: false,
			error: false,
			message: "",
		};
	},

	methods: {
		cameraStart() {
			this.qrScanner.start();
		},

		cameraStop() {
			this.qrScanner.stop();
		},

		updateQrCode(qrCode) {
			if (this.qrCode !== qrCode) {
				fetch("/api/qrcode", {
					method: "POST",
					headers: { "Content-Type": "application/json" },
					body: JSON.stringify(qrCode),
				})
					.then(data => data.json())
					.then(data => {
						this.loading = false;
						this.success = data.success;
						this.error = !data.success;
						this.message = data.message;
					})
					.catch(error => {
						this.loading = false;
						this.success = false;
						this.error = true;
						this.message = error;
					});

				this.loading = true;
				this.qrCode = qrCode;
			}
		},
	},
});
application.mount("#app");
