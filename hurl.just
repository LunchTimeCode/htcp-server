
hurl_opts := "--variables-file hurl.env.test --test"

wait-for-api:
	hurl api_tests/health.hurl --retry 60 {{hurl_opts}}
	
	
# run acceptance tests against the running test stack
api-test *args: wait-for-api
    hurl api_tests/*.hurl {{hurl_opts}} {{args}}	
	