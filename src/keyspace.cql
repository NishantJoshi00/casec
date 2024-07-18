CREATE KEYSPACE IF NOT EXISTS payments
WITH REPLICATION = {
  'class' : 'SimpleStrategy',
  'replication_factor' : 1
};
