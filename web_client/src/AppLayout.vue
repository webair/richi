<template>
  <div class="app-layout">
    <slot />
  </div>
</template>

<style>
.app-layout {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-default);
  padding: 0 var(--spacing-default);
}
</style>
