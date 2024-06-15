import http from 'k6/http';
import { check, sleep } from 'k6';
import { Trend, Rate } from 'k6/metrics';

export const options = {
  vus: 1000,
  duration: '60s',
};

const HomeErrorRate = new Rate('Home errors');
const HelloErrorRate = new Rate('Hello errors');
const HomeTrend = new Trend('Home');
const HelloTrend = new Trend('Hello');

const requests = {
  'Home': {
    method: 'GET',
    url: "http://localhost:3001",
  },
  'Hello': {
    method: 'GET',
    url: "http://localhost:3001/hello",
  },
};


export default function () {
  //http.get('http://localhost:3001/');
  const responses = http.batch(requests);
  const HomeResp = responses['Home'];
  const HelloResp = responses['Hello'];

  check(HomeResp, {
    'status is 200': (r) => r.status === 200,
  }) || HomeErrorRate.add(1);

  HomeTrend.add(HomeResp.timings.duration);

  check(HelloResp, {
    'status is 200': (r) => r.status === 200,
  }) || HelloErrorRate.add(1);

  HelloTrend.add(HelloResp.timings.duration);

  sleep(1);
}
