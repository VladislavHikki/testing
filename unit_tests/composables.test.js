import { test, expect, describe } from 'vitest';


describe('Composables - parseMessage', async () => {
  test('Check parse html symbol', async () => {
    expect(parseMessage('')).toBe('');
    expect(parseMessage('<')).toBe('&lt');
    expect(parseMessage('<script>alert(123123)</script>')).toBe('&ltscript&gtalert(123123)&lt/script&gt');
  });

  test('Check parse link', async () => {
    const url = 'http://localhost:3000/profile/872/chat?user=442';
    expect(parseMessage(url)).toBe(`<a class="link link_color--blue" href="${url}" target="_blank">${url}<a/>`);
  });
});

describe('Composables - onlineDate', async () => {
  test('User exit right now', async () => {
    const date = new Date();

    expect(onlineDate(date)).toBe('был только что');
  });

  test('Show minute', async () => {
    const date = new Date().setMinutes(30);

    expect(onlineDate(date)).toBe('был 30 минут назад');
  });

  test('Show hour', async () => {
    const date = new Date().setHours(2);

    expect(onlineDate(date)).toBe('был 2 часа назад');
  });

  test('Other', async () => {
    const date = new Date(2023,2,19);

    expect(onlineDate(date)).toBe('был 19.03.2023');
  });
});


