"use strict";

const application = Vue.createApp({
  mounted() {
    this.receiptUpdate();
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

    receiptUpdate() {
      fetch("/api/tickets/list", { method: "POST" })
        .then((data) => data.json())
        .then((data) => {
          this.loading = false;
          this.error = false;
          this.items = data.items.map((item) => {
            return {
              categorized: item.type === "Categorized",
              date: item.date,
              product: item.product,
              category: item.category,
              name: item.name,
              quantity: item.quantity.toFixed(3),
              sum: item.sum.toFixed(2),
            };
          });
          this.message = "";
        })
        .catch((error) => {
          this.loading = false;
          this.error = true;
          this.message = error;
          this.items = [];
        });

      this.loading = true;
    },

    receiptClear() {
      fetch("/api/tickets/clear", { method: "POST" })
        .then((data) => data.json())
        .then((data) => {
          this.loading = false;
          this.error = false;
          this.items = [];
          this.message = "";
        })
        .catch((error) => {
          this.loading = false;
          this.error = false;
          this.message = error;
          this.items = [];
        });

      this.loading = true;
    },
  },
});
application.mount("#app");
