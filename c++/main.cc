#include <algorithm>
#include <cstdlib>
#include <iostream>
#include <memory>
#include <random>
#include <vector>

#include <benchmark/benchmark.h>

class Processor {
public:
  virtual int process() = 0;
  virtual ~Processor() {}
};

class Processor0: public Processor {
public:
  int process() { return 1; }
};

class Processor1: public Processor {
public:
  int process() { return 2; }
};

class Processor2: public Processor {
public:
  int process() { return 3; }
};

class Processor3: public Processor {
public:
  int process() { return 5; }
};

class Processor4: public Processor {
public:
  int process() { return 7; }
};

class Processor5: public Processor {
public:
  int process() { return 11; }
};

class Processor6: public Processor {
public:
  int process() { return 13; }
};

class Processor7: public Processor {
public:
  int process() { return 17; }
};

class Processor8: public Processor {
public:
  int process() { return 19; }
};

class Processor9: public Processor {
public:
  int process() { return 23; }
};

class Processor10: public Processor {
public:
  int process() { return 25; }
};

class Processor11: public Processor {
public:
  int process() { return 27; }
};

class Processor12: public Processor {
public:
  int process() { return 29; }
};

class Processor13: public Processor {
public:
  int process() { return 31; }
};

class Processor14: public Processor {
public:
  int process() { return 33; }
};

class Processor15: public Processor {
public:
  int process() { return 35; }
};

class Processor16: public Processor {
public:
  int process() { return 37; }
};

class Processor17: public Processor {
public:
  int process() { return 39; }
};

class Processor18: public Processor {
public:
  int process() { return 41; }
};

class Processor19: public Processor {
public:
  int process() { return 43; }
};

std::unique_ptr<Processor> create(int i) {
  switch (i) {
  case 0: return std::unique_ptr<Processor>(new Processor0());
  case 1: return std::unique_ptr<Processor>(new Processor1());
  case 2: return std::unique_ptr<Processor>(new Processor2());
  case 3: return std::unique_ptr<Processor>(new Processor3());
  case 4: return std::unique_ptr<Processor>(new Processor4());
  case 5: return std::unique_ptr<Processor>(new Processor5());
  case 6: return std::unique_ptr<Processor>(new Processor6());
  case 7: return std::unique_ptr<Processor>(new Processor7());
  case 8: return std::unique_ptr<Processor>(new Processor8());
  case 9: return std::unique_ptr<Processor>(new Processor9());
  case 10: return std::unique_ptr<Processor>(new Processor10());
  case 11: return std::unique_ptr<Processor>(new Processor11());
  case 12: return std::unique_ptr<Processor>(new Processor12());
  case 13: return std::unique_ptr<Processor>(new Processor13());
  case 14: return std::unique_ptr<Processor>(new Processor14());
  case 15: return std::unique_ptr<Processor>(new Processor15());
  case 16: return std::unique_ptr<Processor>(new Processor16());
  case 17: return std::unique_ptr<Processor>(new Processor17());
  case 18: return std::unique_ptr<Processor>(new Processor18());
  case 19: return std::unique_ptr<Processor>(new Processor19());
  }
  return std::unique_ptr<Processor>();
}

int sum(std::vector<std::unique_ptr<Processor> > &data) {
  int result = 0;
  for(const std::unique_ptr<Processor>& item: data) {
    result += item->process();
  }
  return result;
}

static void test_class_1(benchmark::State& state) {
  std::vector<std::unique_ptr<Processor>> data;
  for(int i=0; i < 10000; ++i) {
    data.push_back(create(0));
  }
  for (auto _ : state) {
    sum(data);
  }
}
BENCHMARK(test_class_1);

static void test_class_2(benchmark::State& state) {
  std::vector<std::unique_ptr<Processor>> data;
  for(int i=0; i < 10000; ++i) {
    data.push_back(create(i % 2));
  }
  for (auto _ : state) {
    sum(data);
  }
}
BENCHMARK(test_class_2);

static void test_class_3(benchmark::State& state) {
  std::vector<std::unique_ptr<Processor>> data;
  for(int i=0; i < 10000; ++i) {
    data.push_back(create(i % 3));
  }
  for (auto _ : state) {
    sum(data);
  }
}
BENCHMARK(test_class_3);

static void test_class_4(benchmark::State& state) {
  std::vector<std::unique_ptr<Processor>> data;
  for(int i=0; i < 10000; ++i) {
    data.push_back(create(i % 4));
  }
  for (auto _ : state) {
    sum(data);
  }
}
BENCHMARK(test_class_4);

static void test_class_5(benchmark::State& state) {
  std::vector<std::unique_ptr<Processor>> data;
  for(int i=0; i < 10000; ++i) {
    data.push_back(create(i % 5));
  }
  for (auto _ : state) {
    sum(data);
  }
}
BENCHMARK(test_class_5);

static void test_shuffle_2(benchmark::State& state) {
  std::vector<std::unique_ptr<Processor>> data;
  for(int i=0; i < 10000; ++i) {
    data.push_back(create(i % 2));
  }
  auto rng = std::default_random_engine {};
  std::shuffle(std::begin(data), std::end(data), rng);
  for (auto _ : state) {
    sum(data);
  }
}
BENCHMARK(test_shuffle_2);

static void test_shuffle_3(benchmark::State& state) {
  std::vector<std::unique_ptr<Processor>> data;
  for(int i=0; i < 10000; ++i) {
    data.push_back(create(i % 3));
  }
  auto rng = std::default_random_engine {};
  std::shuffle(std::begin(data), std::end(data), rng);
  for (auto _ : state) {
    sum(data);
  }
}
BENCHMARK(test_shuffle_3);

static void test_shuffle_4(benchmark::State& state) {
  std::vector<std::unique_ptr<Processor>> data;
  for(int i=0; i < 10000; ++i) {
    data.push_back(create(i % 4));
  }
  auto rng = std::default_random_engine {};
  std::shuffle(std::begin(data), std::end(data), rng);
  for (auto _ : state) {
    sum(data);
  }
}
BENCHMARK(test_shuffle_4);

static void test_shuffle_5(benchmark::State& state) {
  std::vector<std::unique_ptr<Processor>> data;
  for(int i=0; i < 10000; ++i) {
    data.push_back(create(i % 5));
  }
  auto rng = std::default_random_engine {};
  std::shuffle(std::begin(data), std::end(data), rng);
  for (auto _ : state) {
    sum(data);
  }
}
BENCHMARK(test_shuffle_5);

BENCHMARK_MAIN();
