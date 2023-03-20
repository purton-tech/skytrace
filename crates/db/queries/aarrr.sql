-- https://ahrefs.com/blog/aarrr-metrics-framework/

--: Metrics()

--! pirate_metrics : Metrics
SELECT COALESCE(SUM(CASE WHEN created_at > current_date - interval '7 days' then 1 end), 0) as registration_past_7_days,
       SUM(1) as total_users
FROM users;