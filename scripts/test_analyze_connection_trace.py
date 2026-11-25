import unittest
from analyze_connection_trace import summarize


class TraceSummaryTest(unittest.TestCase):
    def test_overlapping_spans_and_duplicate_logs_remain_separate(self):
        import json
        def row(span, stage, phase, elapsed, ok=True):
            return 'log OMTRACE ' + json.dumps(dict(v=1, traceId='t', spanId=span,
                 stage=stage, phase=phase, elapsedUs=elapsed, ok=ok))
        lines = [row('a', 'outer', 'start', 0), row('b', 'inner', 'end', 10000),
                 row('a', 'outer', 'headers', 20000), row('a', 'outer', 'end', 30000),
                 row('a', 'outer', 'end', 30000), 'OMTRACE invalid']
        result = {r['stage']: r for r in summarize(lines)}
        self.assertEqual(result['outer/total']['n'], 1)
        self.assertEqual(result['outer/total']['p50_ms'], 30)
        self.assertEqual(result['inner/total']['p50_ms'], 10)
        self.assertEqual(result['outer/headers']['p50_ms'], 20)


if __name__ == '__main__':
    unittest.main()
