import { test, expect, describe } from 'vitest';
import { mountSuspended } from '@nuxt/test-utils/runtime';

import Auth from '~/src/features/user/Auth.vue';

describe('Test component - Auth.vue', async () => {
  const component = await mountSuspended(Auth);

  test('Auth mounted', async () => {
    expect(component.html()).toContain('Email');
  });

  test('Unvisible errros', async () => {
    expect(component.text()).not.toContain('Некорректная почта');
    expect(component.text()).not.toContain('Обязательно');
  })
});
