<!-- MyComponent.vue -->
<template>
  <div class="component-container">
    <div :style="stylee">
      the style props is passed as a props in template. it is {{ stylee }}
      <h2>{{ title }}</h2>
      <p>{{ description }}</p>
      <p>Count: {{ count }}</p>
      <WBC :item="aData" />
      <button @click="handleClick">Click me</button>
    </div>
  </div>
</template>

<script>
export default {
  name: "vue0",
  props: {
    title: {
      type: String,
      required: true,
    },
    description: {
      type: String,
      default: "No description provided",
    },
    stylee: {
      type: Object,
      default: { backgroundColor: "red" },
    },
  },
  data() {
    return {
      count: 0,
      aData: {
        comp: "tex",
        options: {
          class: "blue pa-1 ma-1",
          html: "E = \\sum_a^b \\int_a^b mc^2",
        },
      },
      // Local state can be added here if needed
    };
  },
  methods: {
    handleClick() {
      this.count = this.count + 1;
    },
  },
};
</script>

<style scoped>
.component-container {
  padding: 20px;
  border: 1px solid #ccc;
  border-radius: 5px;
  margin: 10px;
}

h2 {
  color: #2c3e50;
}

button {
  padding: 8px 16px;
  background-color: #42b983;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

button:hover {
  background-color: #3aa876;
}
</style>
