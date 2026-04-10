package com.starkindustries.java_backend.service;

import java.util.Properties;

import org.springframework.stereotype.Service;

import com.starkindustries.java_backend.keys.Keys;

import jakarta.mail.Authenticator;
import jakarta.mail.PasswordAuthentication;
import jakarta.mail.Session;
import jakarta.mail.Transport;
import jakarta.mail.Message.RecipientType;
import jakarta.mail.internet.InternetAddress;
import jakarta.mail.internet.MimeMessage;
import lombok.extern.slf4j.Slf4j;

@Service
@Slf4j
public class EmailService {

    public boolean sendEmail(String toEmail,String name){

    String emailBody = String.format("""
    Hi %s,

    Welcome to our platform!

    We're excited to have you with us.

    Best regards,
    The Team
    """, name);        boolean status = false;

        Properties properties = new Properties();

        properties.put(Keys.HOST, "smtp.gmail.com");
        properties.put(Keys.PORT, "465");
        properties.put(Keys.AUTH, "true");
        properties.put(Keys.SSL_ENABLED, "true");
        properties.put(Keys.SOCKET_FACTORY_PORT, "465");
        properties.put(Keys.SOCKET_FACTORY_CLASS, "javax.net.ssl.SSLSocketFactory");
        properties.put(Keys.SOCKET_FACTORY_FALLBACK, "false");

        Session session = Session.getInstance(properties,new Authenticator() {
            @Override
            protected PasswordAuthentication getPasswordAuthentication() {
                return new PasswordAuthentication(Keys.FROM_EMAIL,Keys.APP_PASSWORD);
            }
        });

        session.setDebug(true);

        try{
            MimeMessage message = new MimeMessage(session);
            message.setFrom(Keys.FROM_EMAIL);
            message.setSubject("Welcome "+name);
            message.setText(emailBody);
            message.addRecipient(RecipientType.TO, new InternetAddress(toEmail));

            Transport.send(message);
            status=true;
            return status;

        }catch(Exception e){
            log.error("Email error: {}",e.getMessage());
            e.printStackTrace();
        }

        return status;

    }
    
}
