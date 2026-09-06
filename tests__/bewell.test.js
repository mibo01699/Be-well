/**
 * اختبارات Be-well Platform API
 */

const request = require('supertest');
const app = require('../app');

describe('Be-well Platform API', () => {
  test('GET /api/health should return 200 OK', async () => {
    const response = await request(app).get('/api/health');
    expect(response.status).toBe(200);
    expect(response.body).toHaveProperty('status', 'UP');
    expect(response.body).toHaveProperty('service', 'Be-well');
  });

  test('GET /api/apps should return app info', async () => {
    const response = await request(app).get('/api/apps');
    expect(response.status).toBe(200);
    expect(response.body).toHaveProperty('id', 'be-well');
    expect(response.body).toHaveProperty('status', 'ONLINE');
  });

  test('GET /api/status should return operational status', async () => {
    const response = await request(app).get('/api/status');
    expect(response.status).toBe(200);
    expect(response.body).toHaveProperty('status', 'OPERATIONAL');
  });

  test('GET / should return welcome message', async () => {
    const response = await request(app).get('/');
    expect(response.status).toBe(200);
    expect(response.body).toHaveProperty('message', '🦅 Be-well Platform API is running');
  });
});