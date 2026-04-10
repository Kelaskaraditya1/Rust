package com.starkindustries.java_backend.configuration;

import com.solacesystems.jms.SolConnectionFactory;
import com.solacesystems.jms.SolJmsUtility;

import javax.jms.ConnectionFactory;

import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.jms.annotation.EnableJms;
import org.springframework.jms.config.DefaultJmsListenerContainerFactory;

@Configuration
@EnableJms
public class SolaceConfig {

    @Value("${solace.host}")
    private String host;

    @Value("${solace.username}")
    private String username;

    @Value("${solace.password}")
    private String password;

    @Value("${solace.vpn}")
    private String vpn;

    @Bean
    public ConnectionFactory connectionFactory() throws Exception {
        SolConnectionFactory cf = SolJmsUtility.createConnectionFactory();
        cf.setHost(host);
        cf.setUsername(username);
        cf.setPassword(password);
        cf.setVPN(vpn);
        return cf;
    }

// @Bean
// public DefaultJmsListenerContainerFactory jmsListenerContainerFactory(
//         ConnectionFactory connectionFactory, 
//         DefaultJmsListenerContainerFactoryConfigurer configurer) {
    
//     DefaultJmsListenerContainerFactory factory = new DefaultJmsListenerContainerFactory();
//     configurer.configure(factory, connectionFactory);
//     factory.setConcurrency("1-3");
//     return factory;
// }
}