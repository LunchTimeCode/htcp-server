
export CLIENT_PORT := "3001"
export SERVER_PORT := "3000"

hurl_opts := "--variables-file hurl.env.test --test"

wait-for-api:
	hurl api_tests/health.hurl --retry 4 {{hurl_opts}}
	
# run acceptance tests against the running test stack
api-test *args: wait-for-api
    hurl api_tests/*.hurl {{hurl_opts}} {{args}}	

# server
run-release: shutdown
    just run-q &  
    
shutdown:
    lsof -t -i:{{SERVER_PORT}} | xargs -r kill    

# test server
@run_test_server_release:
    echo "shutdown server"
    cd test_server && just run-q &

@shutdown_test_server:
    echo "shutdown test server"
    lsof -t -i:{{CLIENT_PORT}} | xargs -r kill    
