<template>
  <form class="ui-form"><slot /></form>
</template>

<style>
.ui-form {
  display: flex;
  align-items: flex-start;
  flex-direction: column;
  gap: var(--spacing-default);

  .error {
    color: var(--color-error);
  }
}
</style>
