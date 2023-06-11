"use strict";

const application = Vue.createApp({
	mounted() {
		this.categoriesUpdate();
	},

	data() {
		return {
			loading: false,
			error: false,
			message: "",
			items: [],
		};
	},

	methods: {
		hasItems() {
			return this.items.length > 0;
		},

		categoryValid(item) {
			return item.category !== "";
		},

		nameValid(item) {
			return item.name !== "";
		},

		itemValid(item) {
			return item.category !== "" && item.name !== "";
		},

		categoriesUpdate() {
			fetch("/api/categories/list", { method: "POST" })
				.then(data => data.json())
				.then(data => {
					this.loading = false;
					this.error = false;
					this.items = data.items.map(item => {
						return {
							product: item.product,
							category: item.category,
							name: item.name,
						};
					});
					this.message = "";
				})
				.catch(error => {
					this.loading = false;
					this.error = false;
					this.message = error;
					this.items = [];
				});

			this.loading = true;
		},

		productUpdate(item) {
			fetch("/api/categories/update", {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify(item),
			})
				.then(data => {
					this.loading = false;
					this.error = false;
					this.message = "";
				})
				.catch(error => {
					this.loading = false;
					this.error = false;
					this.message = error;
				});

			this.loading = true;
		},

		receiptClear() {},
	},
});
application.mount("#app");
