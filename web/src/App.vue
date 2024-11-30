<script setup>
const location = window.location
</script>

<template>
  <div class="h-dvh flex flex-col">
    <div class="flex-1"></div>
    <div class="flex items-center">
      <span class="flex-1"></span>
      <img src="/src/assets/maidono-white.svg" alt="maidono" class="h-24"/>
      <span class="flex-1 pt-4"><span class="badge badge-neutral">v0.1</span></span>
    </div>
    <div class="flex-1">
      <div class="text-center mb-4">
        Your webhook server is running !
      </div>
      <div class="text-center mb-4">
        The root URL for your actions is <span class="text-primary">{{ location }}</span>.
      </div>
    </div>
    <div class="text-center pb-8">
      Sent into orbit by
      <a class="text-error ms-1 relative" href="https://rocket.rs"><span class="glowing bg-danger"></span>
        <fa icon="rocket"/>
        Rocket</a>
    </div>
  </div>
</template>

<style>
.glowing {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  width: 150%;
  height: 150%;
  transition: all 1s ease;
  pointer-events: none;
  z-index: -1;
  background-color: hsl(var(--er) / 0.3);
  border-radius: 50%;
  filter: blur(30px);
}
</style>