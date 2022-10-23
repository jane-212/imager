<template>
  <div class="container">
    <div class="item" v-for="(image, index) in images" :key="index">
      <img class="image" :src="image.url" alt="" />
    </div>
  </div>
</template>

<script>
import get_images from "@/utils/api";

export default {
  data() {
    return {
      images: [],
    };
  },
  methods: {
    load_images: function () {
      let that = this;
      get_images()
        .then((res) => {
          let data = res.data;
          let length = data.length;
          for (let i = 0; i < length; i++) {
            that.images.push(data[i]);
          }
        })
        .catch((err) => {
          console.log(err);
        });
    },
    lazyLoading() {
      let scrollTop =
        document.documentElement.scrollTop || document.body.scrollTop;
      let clientHeight = document.documentElement.clientHeight;
      let scrollHeight = document.documentElement.scrollHeight;
      if (scrollTop + clientHeight >= scrollHeight) {
        this.load_images();
      }
    },
  },
  created() {
    let that = this;
    this.$nextTick(() => {
      that.load_images();
      window.addEventListener("scroll", that.lazyLoading);
    });
  },
  unmounted() {
    window.removeEventListener("scroll", this.lazyLoading);
  },
};
</script>

<style scoped>
.container {
  width: 60vw;
  text-align: center;
  margin: 0 auto;
  overflow: hidden;
  margin-top: 4vh;
}

@media only screen and (max-width: 1024px) {
  .container {
    width: 100vw;
  }
}

.item {
  overflow: hidden;
}

.image {
  display: block;
  max-width: 100%;
}
</style>
