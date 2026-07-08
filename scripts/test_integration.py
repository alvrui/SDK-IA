#!/usr/bin/env python3
"""
Integration test script for SDK-IA project
"""

import asyncio
import aiohttp
import sys


async def test_services():
    """Test all services are running"""
    print("Testing SDK-IA services...")
    
    base_urls = [
        "http://localhost:9090/api/v1/internal/health",
        "http://localhost:9000/api/v1/internal/health",
    ]
    
    async with aiohttp.ClientSession() as session:
        for url in base_urls:
            try:
                async with session.get(url) as response:
                    if response.status == 200:
                        data = await response.json()
                        print(f"Service at {url} is healthy: {data.get('status')}")
                    else:
                        print(f"Service at {url} returned status {response.status}")
                        return False
            except Exception as e:
                print(f"Failed to connect to {url}: {e}")
                return False
    
    return True


async def test_api_endpoints():
    """Test API endpoints"""
    print("Testing API endpoints...")
    
    rust_endpoints = [
        "http://localhost:9090/api/v1/internal/health",
        "http://localhost:9090/api/v1/internal/projects",
    ]
    
    python_endpoints = [
        "http://localhost:9000/api/v1/internal/health",
        "http://localhost:9000/api/v1/internal/agents",
    ]
    
    all_endpoints = rust_endpoints + python_endpoints
    
    async with aiohttp.ClientSession() as session:
        for endpoint in all_endpoints:
            try:
                async with session.get(endpoint) as response:
                    print(f"Endpoint {endpoint} returned status {response.status}")
                    if response.status >= 400:
                        return False
            except Exception as e:
                print(f"Failed to test {endpoint}: {e}")
                return False
    
    return True


async def main():
    """Main test function"""
    print("Starting integration tests...")
    
    if not await test_services():
        print("Service health checks failed!")
        sys.exit(1)
    
    if not await test_api_endpoints():
        print("API endpoint tests failed!")
        sys.exit(1)
    
    print("All integration tests passed!")
    sys.exit(0)


if __name__ == "__main__":
    asyncio.run(main())
