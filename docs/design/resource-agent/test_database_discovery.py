import json
import sys
import types
import unittest
from unittest.mock import Mock, patch
from relay_server import PostgresDeviceStore


class DatabaseDiscoveryTest(unittest.TestCase):
    def test_container_recreate_is_resolved_for_each_connection(self):
        connect = Mock()
        inspect = Mock(side_effect=[
            types.SimpleNamespace(stdout=json.dumps({'cloud': {'IPAddress': '172.18.0.3'}})),
            types.SimpleNamespace(stdout=json.dumps({'cloud': {'IPAddress': '172.18.0.10'}})),
        ])
        with patch.dict(sys.modules, {'psycopg': types.SimpleNamespace(connect=connect)}), patch('relay_server.subprocess.run', inspect):
            store = PostgresDeviceStore('postgresql://unused/db', 'database', 'cloud')
            store._connect()
            store._connect()
        self.assertEqual([c.kwargs['host'] for c in connect.call_args_list], ['172.18.0.3', '172.18.0.10'])
        self.assertTrue(all(c.kwargs['connect_timeout'] == 5 for c in connect.call_args_list))
        self.assertEqual(inspect.call_args.args[0][-1], 'database')

    def test_ambiguous_network_does_not_pick_an_arbitrary_address(self):
        output = types.SimpleNamespace(stdout=json.dumps({'a': {'IPAddress': '172.18.0.3'}, 'b': {'IPAddress': '172.19.0.3'}}))
        connect = Mock()
        with patch.dict(sys.modules, {'psycopg': types.SimpleNamespace(connect=connect)}), patch('relay_server.subprocess.run', return_value=output):
            with self.assertRaises(RuntimeError):
                PostgresDeviceStore('postgresql://unused/db', 'database')._connect()
        connect.assert_not_called()
