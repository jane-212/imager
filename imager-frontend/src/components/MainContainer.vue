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
      is_load: false,
    };
  },
  methods: {
    load_images() {
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
      if (scrollTop + clientHeight >= scrollHeight - clientHeight * 3) {
        if (this.is_load) {
          return;
        }

        this.is_load = true;
        this.load_images();
        this.is_load = false;
      }
    },
    debounce(func, dalay = 100) {
      let timer;
      return (...args) => {
        if (timer) clearTimeout(timer);
        timer = setTimeout(() => {
          func(...args);
        }, dalay);
      };
    },
  },
  created() {
    let that = this;
    this.$nextTick(() => {
      that.load_images();
      window.addEventListener("scroll", that.debounce(that.lazyLoading, 100));
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
  width: 100%;
}

.image {
  display: block;
  width: 100%;
}
</style>
